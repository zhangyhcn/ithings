use axum::{
    extract::{Path, State, Query},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    response::Response,
    service::device_instance::{CreateDeviceInstanceRequest, DeviceInstanceResponse, DeviceInstanceService, UpdateDeviceInstanceRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct DeviceInstancePath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct GroupQuery {
    pub group_id: Option<Uuid>,
}

pub fn create_device_instance_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/device-instances", post(create_device_instance))
        .route("/device-instances", get(list_device_instances))
        .route("/device-instances/:id", get(get_device_instance))
        .route("/device-instances/:id", put(update_device_instance))
        .route("/device-instances/:id", delete(delete_device_instance))
        .with_state(db)
}

async fn create_device_instance(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateDeviceInstanceRequest>,
) -> Result<Json<Response<DeviceInstanceResponse>>, AppError> {
    let service = DeviceInstanceService::new(db);
    let instance = service.create(tenant_id, req).await?;
    Ok(Json(Response::success(instance)))
}

async fn get_device_instance(
    State(db): State<DatabaseConnection>,
    Path(DeviceInstancePath { tenant_id: _, id }): Path<DeviceInstancePath>,
) -> Result<Json<Response<DeviceInstanceResponse>>, AppError> {
    let service = DeviceInstanceService::new(db);
    let instance = service.find_by_id(id).await?;
    Ok(Json(Response::success(instance)))
}

async fn list_device_instances(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Query(query): Query<GroupQuery>,
) -> Result<Json<Response<Vec<DeviceInstanceResponse>>>, AppError> {
    let service = DeviceInstanceService::new(db);
    let instances = if let Some(group_id) = query.group_id {
        service.list_by_group(group_id).await?
    } else {
        service.list_by_tenant(tenant_id).await?
    };
    Ok(Json(Response::success(instances)))
}

async fn update_device_instance(
    State(db): State<DatabaseConnection>,
    Path(DeviceInstancePath { tenant_id: _, id }): Path<DeviceInstancePath>,
    Json(req): Json<UpdateDeviceInstanceRequest>,
) -> Result<Json<Response<DeviceInstanceResponse>>, AppError> {
    let service = DeviceInstanceService::new(db);
    let instance = service.update(id, req).await?;
    Ok(Json(Response::success(instance)))
}

async fn delete_device_instance(
    State(db): State<DatabaseConnection>,
    Path(DeviceInstancePath { tenant_id: _, id }): Path<DeviceInstancePath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = DeviceInstanceService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
