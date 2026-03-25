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
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use api::{
    create_product_router, create_work_order_router, create_material_router,
    create_equipment_router, create_employee_router,
};
use config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("mes=info".parse().unwrap()))
        .init();

    tracing::info!("Starting MES server...");

    let db = Database::connect(&config.database.url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Database connected successfully");

    migration::run_migrations(&db)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database migration completed");

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
        .nest("/tenants/:tenant_id", Router::new()
            .merge(create_product_router(db.clone()))
            .merge(create_work_order_router(db.clone()))
            .merge(create_material_router(db.clone()))
            .merge(create_equipment_router(db.clone()))
            .merge(create_employee_router(db))
        );

    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1/mes", api_v1)
}

async fn health_check() -> &'static str {
    "OK"
}
