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

#[derive(Debug, Deserialize)]
pub struct ListTagsQuery {
    pub registry: Option<String>,
    pub image: String,
}

pub fn create_driver_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/drivers", post(create_driver))
        .route("/drivers", get(list_drivers))
        .route("/drivers/:id", get(get_driver))
        .route("/drivers/:id", put(update_driver))
        .route("/drivers/:id", delete(delete_driver))
        .route("/drivers/tags", get(list_image_tags))
        .route("/drivers/images", get(list_registry_images))
        .with_state(db)
}

async fn list_image_tags(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Query(query): Query<ListTagsQuery>,
) -> Result<Json<Response<Vec<String>>>, AppError> {
    let service = DriverService::new(db);
    let tags = service.list_image_tags(tenant_id, query.registry, query.image).await?;
    Ok(Json(Response::success(tags)))
}

async fn list_registry_images(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<Json<Response<Vec<DriverResponse>>>, AppError> {
    let service = DriverService::new(db);
    let images = service.list_registry_images(tenant_id).await?;
    Ok(Json(Response::success(images)))
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
    tracing::info!("API list_drivers called with tenant_id: {}", tenant_id);
    let service = DriverService::new(db);
    let drivers = service.list_by_tenant(tenant_id).await?;
    tracing::info!("API list_drivers returning {} drivers", drivers.len());
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
