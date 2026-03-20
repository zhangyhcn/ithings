use axum::{
    extract::{Path, State, Query},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    response::Response,
    service::driver::{CreateDriverRequest, DriverResponse, DriverService, UpdateDriverRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct DriverPath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

pub fn create_driver_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/drivers", post(create_driver))
        .route("/drivers", get(list_drivers))
        .route("/drivers/:id", get(get_driver))
        .route("/drivers/:id", put(update_driver))
        .route("/drivers/:id", delete(delete_driver))
        .with_state(db)
}

async fn create_driver(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateDriverRequest>,
) -> Result<Json<Response<DriverResponse>>, AppError> {
    let service = DriverService::new(db);
    let driver = service.create(tenant_id, req).await?;
    Ok(Json(Response::success(driver)))
}

async fn get_driver(
    State(db): State<DatabaseConnection>,
    Path(DriverPath { tenant_id: _, id }): Path<DriverPath>,
) -> Result<Json<Response<DriverResponse>>, AppError> {
    let service = DriverService::new(db);
    let driver = service.find_by_id(id).await?;
    Ok(Json(Response::success(driver)))
}

async fn list_drivers(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Query(_query): Query<PageQuery>,
) -> Result<Json<Response<Vec<DriverResponse>>>, AppError> {
    let service = DriverService::new(db);
    let drivers = service.list_by_tenant(tenant_id).await?;
    Ok(Json(Response::success(drivers)))
}

async fn update_driver(
    State(db): State<DatabaseConnection>,
    Path(DriverPath { tenant_id: _, id }): Path<DriverPath>,
    Json(req): Json<UpdateDriverRequest>,
) -> Result<Json<Response<DriverResponse>>, AppError> {
    let service = DriverService::new(db);
    let driver = service.update(id, req).await?;
    Ok(Json(Response::success(driver)))
}

async fn delete_driver(
    State(db): State<DatabaseConnection>,
    Path(DriverPath { tenant_id: _, id }): Path<DriverPath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = DriverService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
