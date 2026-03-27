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
    service::device_group::{CreateDeviceGroupRequest, DeviceGroupResponse, DeviceGroupService, UpdateDeviceGroupRequest, PublishDeviceGroupRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct DeviceGroupPath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

pub fn create_device_group_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/device-groups", post(create_device_group))
        .route("/device-groups", get(list_device_groups))
        .route("/device-groups/:id", get(get_device_group))
        .route("/device-groups/:id", put(update_device_group))
        .route("/device-groups/:id", delete(delete_device_group))
        .route("/device-groups/:id/publish", post(publish_device_group))
        .with_state(db)
}

async fn create_device_group(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateDeviceGroupRequest>,
) -> Result<Json<Response<DeviceGroupResponse>>, AppError> {
    let service = DeviceGroupService::new(db);
    let group = service.create(tenant_id, req).await?;
    Ok(Json(Response::success(group)))
}

async fn get_device_group(
    State(db): State<DatabaseConnection>,
    Path(DeviceGroupPath { tenant_id: _, id }): Path<DeviceGroupPath>,
) -> Result<Json<Response<DeviceGroupResponse>>, AppError> {
    let service = DeviceGroupService::new(db);
    let group = service.find_by_id(id).await?;
    Ok(Json(Response::success(group)))
}

async fn list_device_groups(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<Json<Response<Vec<DeviceGroupResponse>>>, AppError> {
    let service = DeviceGroupService::new(db);
    let groups = service.list_by_tenant(tenant_id).await?;
    Ok(Json(Response::success(groups)))
}

async fn update_device_group(
    State(db): State<DatabaseConnection>,
    Path(DeviceGroupPath { tenant_id: _, id }): Path<DeviceGroupPath>,
    Json(req): Json<UpdateDeviceGroupRequest>,
) -> Result<Json<Response<DeviceGroupResponse>>, AppError> {
    let service = DeviceGroupService::new(db);
    let group = service.update(id, req).await?;
    Ok(Json(Response::success(group)))
}

async fn delete_device_group(
    State(db): State<DatabaseConnection>,
    Path(DeviceGroupPath { tenant_id: _, id }): Path<DeviceGroupPath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = DeviceGroupService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}

async fn publish_device_group(
    State(db): State<DatabaseConnection>,
    Path(DeviceGroupPath { tenant_id: _, id }): Path<DeviceGroupPath>,
    Json(req): Json<PublishDeviceGroupRequest>,
) -> Result<Json<Response<()>>, AppError> {
    let service = DeviceGroupService::new(db);
    service.publish(id, req).await?;
    Ok(Json(Response::success(())))
}
