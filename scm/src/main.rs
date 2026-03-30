mod api;
mod config;
mod entity;
mod migration;
mod response;
mod service;
mod utils;

use axum::{
    routing::get,
    Router,
};
use sea_orm::Database;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use api::{
    create_supplier_router, create_purchase_order_router,
    create_material_router, create_warehouse_router, create_inventory_router,
    create_inbound_order_router, create_outbound_order_router,
    create_supplier_quotation_router, create_bidding_router, create_contract_router,
    create_stocktaking_router,
};
use config::Config;

async fn init_database_tables(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Initializing database tables...");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;
    
    let sql = include_str!("../init_tables.sql");
    
    // 执行整个SQL文件
    sqlx::raw_sql(sql)
        .execute(&pool)
        .await?;
    
    tracing::info!("Database tables initialized successfully");
    Ok(())
}

#[tokio::main]
async fn main() {
    let config = Config::load();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("scm=info".parse().unwrap()))
        .init();

    tracing::info!("Starting SCM server...");

    let db = Database::connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connected successfully");

    // 自动初始化数据库表
    init_database_tables(&config.database.url)
        .await
        .expect("Failed to initialize database tables");

    let app = create_app(db.clone());

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

fn create_app(db: sea_orm::DatabaseConnection) -> Router {
    let api_v1 = Router::new()
        .nest("/tenants/:tenant_id/orgs/:org_id", Router::new()
            .merge(create_supplier_router(db.clone()))
            .merge(create_purchase_order_router(db.clone()))
            .merge(create_material_router(db.clone()))
            .merge(create_warehouse_router(db.clone()))
            .merge(create_inventory_router(db.clone()))
            .merge(create_inbound_order_router(db.clone()))
            .merge(create_outbound_order_router(db.clone()))
            .merge(create_supplier_quotation_router(db.clone()))
            .merge(create_bidding_router(db.clone()))
            .merge(create_contract_router(db.clone()))
            .merge(create_stocktaking_router(db))
        );

    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1/scm", api_v1)
}

async fn health_check() -> &'static str {
    "OK"
}
