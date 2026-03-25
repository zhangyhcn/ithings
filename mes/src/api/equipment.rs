use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
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

pub fn create_equipment_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/equipments", get(list_equipments).post(create_equipment))
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
