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
    service::material::{MaterialService, CreateMaterialRequest, MaterialResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMaterialRequest {
    pub material_name: Option<String>,
    pub specification: Option<String>,
    pub unit: Option<String>,
    pub material_type: Option<String>,
    pub safety_stock: Option<f64>,
    pub max_stock: Option<f64>,
    pub status: Option<String>,
}

pub fn create_material_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/materials", get(list_materials).post(create_material))
        .route("/materials/:id", get(get_material).put(update_material).delete(delete_material))
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

async fn get_material(
    State(db): State<DatabaseConnection>,
    Path((TenantPath { tenant_id }, id)): Path<(TenantPath, Uuid)>,
) -> Result<ResponseJson<Response<MaterialResponse>>, AppError> {
    let service = MaterialService::new(db);
    let material = service.get_by_id(tenant_id, id).await?
        .ok_or_else(|| AppError::not_found("Material not found".to_string()))?;
    Ok(ResponseJson(Response::success(material)))
}

async fn update_material(
    State(db): State<DatabaseConnection>,
    Path((TenantPath { tenant_id }, id)): Path<(TenantPath, Uuid)>,
    Json(req): Json<UpdateMaterialRequest>,
) -> Result<ResponseJson<Response<MaterialResponse>>, AppError> {
    let service = MaterialService::new(db);
    let material = service.update(
        tenant_id,
        id,
        req.material_name,
        req.specification,
        req.unit,
        req.material_type,
        req.safety_stock,
        req.max_stock,
        req.status,
    ).await?;
    Ok(ResponseJson(Response::success(material)))
}

async fn delete_material(
    State(db): State<DatabaseConnection>,
    Path((TenantPath { tenant_id }, id)): Path<(TenantPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = MaterialService::new(db);
    service.delete(tenant_id, id).await?;
    Ok(ResponseJson(Response::success(())))
}
