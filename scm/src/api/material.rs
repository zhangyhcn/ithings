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
    service::material::{MaterialService, CreateMaterialRequest, UpdateMaterialRequest, MaterialResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_material_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/materials", get(list_materials).post(create_material))
        .route("/materials/:id", get(get_material).put(update_material).delete(delete_material))
        .with_state(db)
}

async fn list_materials(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<MaterialResponse>>>, AppError> {
    let service = MaterialService::new(db);
    let materials = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(materials)))
}

async fn create_material(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreateMaterialRequest>,
) -> Result<ResponseJson<Response<MaterialResponse>>, AppError> {
    let service = MaterialService::new(db);
    let material = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(material)))
}

async fn get_material(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<MaterialResponse>>, AppError> {
    let service = MaterialService::new(db);
    let material = service.get_by_id(tenant_id, org_id, id).await?
        .ok_or_else(|| AppError::not_found("Material not found".to_string()))?;
    Ok(ResponseJson(Response::success(material)))
}

async fn update_material(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
    Json(req): Json<UpdateMaterialRequest>,
) -> Result<ResponseJson<Response<MaterialResponse>>, AppError> {
    let service = MaterialService::new(db);
    let material = service.update(tenant_id, org_id, id, req).await?;
    Ok(ResponseJson(Response::success(material)))
}

async fn delete_material(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = MaterialService::new(db);
    service.delete(tenant_id, org_id, id).await?;
    Ok(ResponseJson(Response::success(())))
}
