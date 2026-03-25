use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    response::Response,
    service::work_order::{WorkOrderService, CreateWorkOrderRequest, WorkOrderResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct WorkOrderPath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompleteRequest {
    pub completed_qty: f64,
}

pub fn create_work_order_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/work-orders", get(list_work_orders).post(create_work_order))
        .route("/work-orders/:id", get(get_work_order))
        .route("/work-orders/:id/start", post(start_work_order))
        .route("/work-orders/:id/complete", post(complete_work_order))
        .with_state(db)
}

async fn list_work_orders(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<ResponseJson<Response<Vec<WorkOrderResponse>>>, AppError> {
    let service = WorkOrderService::new(db);
    let work_orders = service.list_all(tenant_id).await?;
    Ok(ResponseJson(Response::success(work_orders)))
}

async fn create_work_order(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateWorkOrderRequest>,
) -> Result<ResponseJson<Response<WorkOrderResponse>>, AppError> {
    let service = WorkOrderService::new(db);
    let work_order = service.create(tenant_id, req).await?;
    Ok(ResponseJson(Response::success(work_order)))
}

async fn get_work_order(
    State(db): State<DatabaseConnection>,
    Path(WorkOrderPath { tenant_id: _, id }): Path<WorkOrderPath>,
) -> Result<ResponseJson<Response<WorkOrderResponse>>, AppError> {
    let service = WorkOrderService::new(db);
    let work_order = service.find_by_id(id).await?;
    Ok(ResponseJson(Response::success(work_order)))
}

async fn start_work_order(
    State(db): State<DatabaseConnection>,
    Path(WorkOrderPath { tenant_id: _, id }): Path<WorkOrderPath>,
) -> Result<ResponseJson<Response<WorkOrderResponse>>, AppError> {
    let service = WorkOrderService::new(db);
    let work_order = service.start(id).await?;
    Ok(ResponseJson(Response::success(work_order)))
}

async fn complete_work_order(
    State(db): State<DatabaseConnection>,
    Path(WorkOrderPath { tenant_id: _, id }): Path<WorkOrderPath>,
    Json(req): Json<CompleteRequest>,
) -> Result<ResponseJson<Response<WorkOrderResponse>>, AppError> {
    let service = WorkOrderService::new(db);
    let work_order = service.complete(id, req.completed_qty).await?;
    Ok(ResponseJson(Response::success(work_order)))
}
