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
    service::device_instance::{CreateDeviceInstanceRequest, DeviceInstanceResponse, DeviceInstanceService, UpdateDeviceInstanceRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct SitePath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
    pub site_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

pub fn create_device_instance_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/device-instances", post(create_device_instance))
        .route("/device-instances", get(list_device_instances))
        .route("/device-instances/{id}", get(get_device_instance))
        .route("/device-instances/{id}", put(update_device_instance))
        .route("/device-instances/{id}", delete(delete_device_instance))
        .with_state(db)
}

async fn create_device_instance(
    State(db): State<DatabaseConnection>,
    Path(SitePath { tenant_id, org_id, site_id }): Path<SitePath>,
    Json(req): Json<CreateDeviceInstanceRequest>,
) -> Result<Json<Response<DeviceInstanceResponse>>, AppError> {
    let service = DeviceInstanceService::new(db);
    let device_instance = service.create(
        tenant_id,
        org_id,
        site_id,
        CreateDeviceInstanceRequest {
            name: req.name,
            brand_model: req.brand_model,
            product_id: req.product_id,
            driver_id: req.driver_id,
            poll_interval_ms: req.poll_interval_ms,
            device_type: req.device_type,
            driver_config: req.driver_config,
            thing_model: req.thing_model,
            node_id: req.node_id,
        }
    ).await?;
    Ok(Json(Response::success(device_instance)))
}

async fn get_device_instance(
    State(db): State<DatabaseConnection>,
    Path((_tenant_id, _org_id, _site_id, id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
) -> Result<Json<Response<DeviceInstanceResponse>>, AppError> {
    let service = DeviceInstanceService::new(db);
    let device_instance = service.find_by_id(id).await?;
    Ok(Json(Response::success(device_instance)))
}

async fn list_device_instances(
    State(db): State<DatabaseConnection>,
    Path(SitePath { tenant_id, org_id, site_id }): Path<SitePath>,
    Query(_query): Query<PageQuery>,
) -> Result<Json<Response<Vec<DeviceInstanceResponse>>>, AppError> {
    let service = DeviceInstanceService::new(db);
    let devices = service.list_by_site(tenant_id, org_id, site_id).await?;
    Ok(Json(Response::success(devices)))
}

async fn update_device_instance(
    State(db): State<DatabaseConnection>,
    Path((_tenant_id, _org_id, _site_id, id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
    Json(req): Json<UpdateDeviceInstanceRequest>,
) -> Result<Json<Response<DeviceInstanceResponse>>, AppError> {
    let service = DeviceInstanceService::new(db);
    let device_instance = service.update(id, req).await?;
    Ok(Json(Response::success(device_instance)))
}

async fn delete_device_instance(
    State(db): State<DatabaseConnection>,
    Path((_tenant_id, _org_id, _site_id, id)): Path<(Uuid, Uuid, Uuid, Uuid)>,
) -> Result<Json<Response<()>>, AppError> {
    let service = DeviceInstanceService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
