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
    service::device::{CreateDeviceRequest, DeviceResponse, DeviceService, UpdateDeviceRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct DevicePath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

pub fn create_device_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/devices", post(create_device))
        .route("/devices", get(list_devices))
        .route("/devices/:id", get(get_device))
        .route("/devices/:id", put(update_device))
        .route("/devices/:id", delete(delete_device))
        .with_state(db)
}

async fn create_device(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateDeviceRequest>,
) -> Result<Json<Response<DeviceResponse>>, AppError> {
    let service = DeviceService::new(db);
    let device = service.create(tenant_id, req).await?;
    Ok(Json(Response::success(device)))
}

async fn get_device(
    State(db): State<DatabaseConnection>,
    Path(DevicePath { tenant_id: _, id }): Path<DevicePath>,
) -> Result<Json<Response<DeviceResponse>>, AppError> {
    let service = DeviceService::new(db);
    let device = service.find_by_id(id).await?;
    Ok(Json(Response::success(device)))
}

async fn list_devices(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Query(_query): Query<PageQuery>,
) -> Result<Json<Response<Vec<DeviceResponse>>>, AppError> {
    let service = DeviceService::new(db);
    let devices = service.list_by_tenant(tenant_id).await?;
    Ok(Json(Response::success(devices)))
}

async fn update_device(
    State(db): State<DatabaseConnection>,
    Path(DevicePath { tenant_id: _, id }): Path<DevicePath>,
    Json(req): Json<UpdateDeviceRequest>,
) -> Result<Json<Response<DeviceResponse>>, AppError> {
    let service = DeviceService::new(db);
    let device = service.update(id, req).await?;
    Ok(Json(Response::success(device)))
}

async fn delete_device(
    State(db): State<DatabaseConnection>,
    Path(DevicePath { tenant_id: _, id }): Path<DevicePath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = DeviceService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
