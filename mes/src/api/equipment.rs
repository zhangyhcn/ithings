use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    response::Response,
    service::equipment::{EquipmentService, CreateEquipmentRequest, EquipmentResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEquipmentRequest {
    pub equipment_name: Option<String>,
    pub equipment_type: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub purchase_date: Option<String>,
    pub workshop_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub status: Option<String>,
}

pub fn create_equipment_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/equipments", get(list_equipments).post(create_equipment))
        .route("/equipments/:id", get(get_equipment).put(update_equipment).delete(delete_equipment))
        .with_state(db)
}

async fn list_equipments(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<ResponseJson<Response<Vec<EquipmentResponse>>>, AppError> {
    let service = EquipmentService::new(db);
    let equipments = service.list_all(tenant_id).await?;
    Ok(ResponseJson(Response::success(equipments)))
}

async fn create_equipment(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateEquipmentRequest>,
) -> Result<ResponseJson<Response<EquipmentResponse>>, AppError> {
    let service = EquipmentService::new(db);
    let equipment = service.create(tenant_id, req).await?;
    Ok(ResponseJson(Response::success(equipment)))
}

async fn get_equipment(
    State(db): State<DatabaseConnection>,
    Path((TenantPath { tenant_id }, id)): Path<(TenantPath, Uuid)>,
) -> Result<ResponseJson<Response<EquipmentResponse>>, AppError> {
    let service = EquipmentService::new(db);
    let equipment = service.get_by_id(tenant_id, id).await?
        .ok_or_else(|| AppError::not_found("Equipment not found".to_string()))?;
    Ok(ResponseJson(Response::success(equipment)))
}

async fn update_equipment(
    State(db): State<DatabaseConnection>,
    Path((TenantPath { tenant_id }, id)): Path<(TenantPath, Uuid)>,
    Json(req): Json<UpdateEquipmentRequest>,
) -> Result<ResponseJson<Response<EquipmentResponse>>, AppError> {
    let service = EquipmentService::new(db);
    let equipment = service.update(
        tenant_id,
        id,
        req.equipment_name,
        req.equipment_type,
        req.model,
        req.manufacturer,
        req.purchase_date,
        req.workshop_id,
        req.ip_address,
        req.status,
    ).await?;
    Ok(ResponseJson(Response::success(equipment)))
}

async fn delete_equipment(
    State(db): State<DatabaseConnection>,
    Path((TenantPath { tenant_id }, id)): Path<(TenantPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = EquipmentService::new(db);
    service.delete(tenant_id, id).await?;
    Ok(ResponseJson(Response::success(())))
}
