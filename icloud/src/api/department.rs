use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    response::Response,
    service::department::{CreateDepartmentRequest, DepartmentResponse, DepartmentService, UpdateDepartmentRequest},
    utils::AppError,
};

pub fn create_department_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/organizations/{org_id}/departments", post(create_department))
        .route("/organizations/{org_id}/departments", get(list_departments))
        .route("/organizations/{org_id}/departments/{id}", get(get_department))
        .route("/organizations/{org_id}/departments/{id}", put(update_department))
        .route("/organizations/{org_id}/departments/{id}", delete(delete_department))
        .with_state(db)
}

async fn create_department(
    State(db): State<DatabaseConnection>,
    Path(OrganizationPath { org_id }): Path<OrganizationPath>,
    Json(req): Json<CreateDepartmentRequest>,
) -> Result<Json<Response<DepartmentResponse>>, AppError> {
    let service = DepartmentService::new(db);
    let dept = service.create(org_id, req).await?;
    Ok(Json(Response::success(dept)))
}

async fn get_department(
    State(db): State<DatabaseConnection>,
    Path(DepartmentPath { org_id, id }): Path<DepartmentPath>,
) -> Result<Json<Response<DepartmentResponse>>, AppError> {
    let service = DepartmentService::new(db);
    let dept = service.find_by_id(org_id, id).await?;
    Ok(Json(Response::success(dept)))
}

#[derive(Debug, Deserialize)]
pub struct OrganizationPath {
    pub org_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct DepartmentPath {
    pub org_id: Uuid,
    pub id: Uuid,
}

async fn list_departments(
    State(db): State<DatabaseConnection>,
    Path(OrganizationPath { org_id }): Path<OrganizationPath>,
) -> Result<Json<Response<Vec<DepartmentResponse>>>, AppError> {
    let service = DepartmentService::new(db);
    let depts = service.list(org_id).await?;
    Ok(Json(Response::success(depts)))
}

async fn update_department(
    State(db): State<DatabaseConnection>,
    Path(DepartmentPath { org_id, id }): Path<DepartmentPath>,
    Json(req): Json<UpdateDepartmentRequest>,
) -> Result<Json<Response<DepartmentResponse>>, AppError> {
    let service = DepartmentService::new(db);
    let dept = service.update(org_id, id, req).await?;
    Ok(Json(Response::success(dept)))
}

async fn delete_department(
    State(db): State<DatabaseConnection>,
    Path(DepartmentPath { org_id, id }): Path<DepartmentPath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = DepartmentService::new(db);
    service.delete(org_id, id).await?;
    Ok(Json(Response::success(())))
}
