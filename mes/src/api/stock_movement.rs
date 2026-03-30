use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    response::Response,
    service::stock_movement::{StockMovementService, CreateStockMovementRequest, StockMovementResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct StockMovementPath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

pub fn create_stock_movement_router(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/stock-movements", get(list_stock_movements).post(create_stock_movement))
        .route("/stock-movements/:id", get(get_stock_movement))
        .route("/stock-movements/:id/execute", post(execute_stock_movement))
        .route("/stock-movements/:id/cancel", post(cancel_stock_movement))
        .with_state(db)
}

async fn list_stock_movements(
    State(db): State<Arc<DatabaseConnection>>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<ResponseJson<Response<Vec<StockMovementResponse>>>, AppError> {
    let service = StockMovementService::new(db);
    let movements = service.list_all(tenant_id).await?;
    Ok(ResponseJson(Response::success(movements)))
}

async fn create_stock_movement(
    State(db): State<Arc<DatabaseConnection>>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateStockMovementRequest>,
) -> Result<ResponseJson<Response<StockMovementResponse>>, AppError> {
    let service = StockMovementService::new(db);
    let movement = service.create(tenant_id, req).await?;
    Ok(ResponseJson(Response::success(movement)))
}

async fn get_stock_movement(
    State(db): State<Arc<DatabaseConnection>>,
    Path(StockMovementPath { tenant_id: _, id }): Path<StockMovementPath>,
) -> Result<ResponseJson<Response<StockMovementResponse>>, AppError> {
    let service = StockMovementService::new(db);
    let movement = service.find_by_id(id).await?;
    Ok(ResponseJson(Response::success(movement)))
}

async fn execute_stock_movement(
    State(db): State<Arc<DatabaseConnection>>,
    Path(StockMovementPath { tenant_id: _, id }): Path<StockMovementPath>,
) -> Result<ResponseJson<Response<StockMovementResponse>>, AppError> {
    let service = StockMovementService::new(db);
    let movement = service.execute(id).await?;
    Ok(ResponseJson(Response::success(movement)))
}

async fn cancel_stock_movement(
    State(db): State<Arc<DatabaseConnection>>,
    Path(StockMovementPath { tenant_id: _, id }): Path<StockMovementPath>,
) -> Result<ResponseJson<Response<StockMovementResponse>>, AppError> {
    let service = StockMovementService::new(db);
    let movement = service.cancel(id).await?;
    Ok(ResponseJson(Response::success(movement)))
}
