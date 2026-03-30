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
    service::inventory::{InventoryService, AdjustInventoryRequest, LockInventoryRequest, InventoryResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct InventoryPath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct MaterialPath {
    pub tenant_id: Uuid,
    pub material_id: Uuid,
}

pub fn create_inventory_router(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/inventories", get(list_inventories))
        .route("/inventories/:id", get(get_inventory))
        .route("/inventories/adjust", post(adjust_inventory))
        .route("/inventories/lock", post(lock_inventory))
        .route("/inventories/unlock", post(unlock_inventory))
        .route("/materials/:material_id/inventories", get(list_inventories_by_material))
        .with_state(db)
}

async fn list_inventories(
    State(db): State<Arc<DatabaseConnection>>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<ResponseJson<Response<Vec<InventoryResponse>>>, AppError> {
    let service = InventoryService::new(db);
    let inventories = service.list_all(tenant_id).await?;
    Ok(ResponseJson(Response::success(inventories)))
}

async fn list_inventories_by_material(
    State(db): State<Arc<DatabaseConnection>>,
    Path(MaterialPath { tenant_id, material_id }): Path<MaterialPath>,
) -> Result<ResponseJson<Response<Vec<InventoryResponse>>>, AppError> {
    let service = InventoryService::new(db);
    let inventories = service.list_by_material(tenant_id, material_id).await?;
    Ok(ResponseJson(Response::success(inventories)))
}

async fn get_inventory(
    State(db): State<Arc<DatabaseConnection>>,
    Path(InventoryPath { tenant_id: _, id }): Path<InventoryPath>,
) -> Result<ResponseJson<Response<InventoryResponse>>, AppError> {
    let service = InventoryService::new(db);
    let inventory = service.find_by_id(id).await?;
    Ok(ResponseJson(Response::success(inventory)))
}

async fn adjust_inventory(
    State(db): State<Arc<DatabaseConnection>>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<AdjustInventoryRequest>,
) -> Result<ResponseJson<Response<InventoryResponse>>, AppError> {
    let service = InventoryService::new(db);
    let inventory = service.adjust(tenant_id, req).await?;
    Ok(ResponseJson(Response::success(inventory)))
}

async fn lock_inventory(
    State(db): State<Arc<DatabaseConnection>>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<LockInventoryRequest>,
) -> Result<ResponseJson<Response<InventoryResponse>>, AppError> {
    let service = InventoryService::new(db);
    let inventory = service.lock(tenant_id, req).await?;
    Ok(ResponseJson(Response::success(inventory)))
}

#[derive(Debug, Deserialize)]
pub struct UnlockRequest {
    pub material_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub batch_no: Option<String>,
    pub quantity: f64,
}

async fn unlock_inventory(
    State(db): State<Arc<DatabaseConnection>>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<UnlockRequest>,
) -> Result<ResponseJson<Response<InventoryResponse>>, AppError> {
    let service = InventoryService::new(db);
    let inventory = service.unlock(
        tenant_id,
        req.material_id,
        req.warehouse_id,
        req.location_id,
        req.batch_no,
        req.quantity,
    ).await?;
    Ok(ResponseJson(Response::success(inventory)))
}
