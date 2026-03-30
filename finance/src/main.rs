use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::Database;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod entity;
mod service;

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub config: Config,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "finance=debug,sqlx=warn".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 加载配置
    let config = Config::load()?;
    tracing::info!("Loaded configuration");

    // 初始化数据库连接池
    let _pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;
    tracing::info!("Database pool created");

    // 初始化数据库表
    init_database_tables(&config.database_url).await?;
    tracing::info!("Database tables initialized");

    // 创建 SeaORM 数据库连接
    let db = Database::connect(&config.database_url).await?;
    tracing::info!("SeaORM database connection established");

    // 创建应用状态
    let state = Arc::new(AppState { db, config });

    // 创建路由
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        // Account API
        .route("/api/v1/finance/accounts", get(api::account::list_accounts).post(api::account::create_account))
        .route("/api/v1/finance/accounts/:id", get(api::account::get_account).delete(api::account::delete_account))
        // Voucher API
        .route("/api/v1/finance/vouchers", get(api::voucher::list_vouchers).post(api::voucher::create_voucher))
        .route("/api/v1/finance/vouchers/:id", get(api::voucher::get_voucher).delete(api::voucher::delete_voucher))
        .route("/api/v1/finance/vouchers/:id/approve", post(api::voucher::approve_voucher))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // 启动服务器
    let addr = format!("0.0.0.0:{}", 8082);
    tracing::info!("Finance service listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn init_database_tables(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new().max_connections(5).connect(db_url).await?;
    let sql = include_str!("../init_tables.sql");
    sqlx::raw_sql(sql).execute(&pool).await?;
    tracing::info!("Database tables initialized successfully");
    Ok(())
}
