use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    response::Response,
    service::warehouse::{WarehouseService, CreateWarehouseRequest, UpdateWarehouseRequest, WarehouseResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_warehouse_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/warehouses", get(list_warehouses).post(create_warehouse))
        .route("/warehouses/:id", get(get_warehouse).put(update_warehouse).delete(delete_warehouse))
        .with_state(db)
}

async fn list_warehouses(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<WarehouseResponse>>>, AppError> {
    let service = WarehouseService::new(db);
    let warehouses = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(warehouses)))
}

async fn create_warehouse(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreateWarehouseRequest>,
) -> Result<ResponseJson<Response<WarehouseResponse>>, AppError> {
    let service = WarehouseService::new(db);
    let warehouse = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(warehouse)))
}

async fn get_warehouse(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<WarehouseResponse>>, AppError> {
    let service = WarehouseService::new(db);
    let warehouse = service.get_by_id(tenant_id, org_id, id).await?
        .ok_or_else(|| AppError::not_found("Warehouse not found".to_string()))?;
    Ok(ResponseJson(Response::success(warehouse)))
}

async fn update_warehouse(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
    Json(req): Json<UpdateWarehouseRequest>,
) -> Result<ResponseJson<Response<WarehouseResponse>>, AppError> {
    let service = WarehouseService::new(db);
    let warehouse = service.update(tenant_id, org_id, id, req).await?;
    Ok(ResponseJson(Response::success(warehouse)))
}

async fn delete_warehouse(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = WarehouseService::new(db);
    service.delete(tenant_id, org_id, id).await?;
    Ok(ResponseJson(Response::success(())))
}
