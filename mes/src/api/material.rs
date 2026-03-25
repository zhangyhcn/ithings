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
    service::material::{MaterialService, CreateMaterialRequest, MaterialResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

pub fn create_material_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/materials", get(list_materials).post(create_material))
        .with_state(db)
}

async fn list_materials(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<ResponseJson<Response<Vec<MaterialResponse>>>, AppError> {
    let service = MaterialService::new(db);
    let materials = service.list_all(tenant_id).await?;
    Ok(ResponseJson(Response::success(materials)))
}

async fn create_material(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateMaterialRequest>,
) -> Result<ResponseJson<Response<MaterialResponse>>, AppError> {
    let service = MaterialService::new(db);
    let material = service.create(tenant_id, req).await?;
    Ok(ResponseJson(Response::success(material)))
}
