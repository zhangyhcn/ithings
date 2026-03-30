use axum::{
    extract::{Path, State},
    response::Json as ResponseJson,
    routing::get,
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    response::Response,
    service::inventory::InventoryService,
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_inventory_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/inventory", get(list_inventory))
        .route("/inventory/warehouse/:warehouse_id", get(list_inventory_by_warehouse))
        .route("/inventory/material/:material_id", get(list_inventory_by_material))
        .route("/inventory/:id", get(get_inventory))
        .with_state(db)
}

async fn list_inventory(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<crate::service::inventory::InventoryResponse>>>, AppError> {
    let service = InventoryService::new(db);
    let inventory = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(inventory)))
}

async fn list_inventory_by_warehouse(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, warehouse_id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<Vec<crate::service::inventory::InventoryResponse>>>, AppError> {
    let service = InventoryService::new(db);
    let inventory = service.list_by_warehouse(tenant_id, org_id, warehouse_id).await?;
    Ok(ResponseJson(Response::success(inventory)))
}

async fn list_inventory_by_material(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, material_id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<Vec<crate::service::inventory::InventoryResponse>>>, AppError> {
    let service = InventoryService::new(db);
    let inventory = service.list_by_material(tenant_id, org_id, material_id).await?;
    Ok(ResponseJson(Response::success(inventory)))
}

async fn get_inventory(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<crate::service::inventory::InventoryResponse>>, AppError> {
    let service = InventoryService::new(db);
    let inventory = service.get_by_id(tenant_id, org_id, id).await?
        .ok_or_else(|| AppError::not_found("Inventory not found".to_string()))?;
    Ok(ResponseJson(Response::success(inventory)))
}
