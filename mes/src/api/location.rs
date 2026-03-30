use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    Router,
    routing::{get, post, put, delete},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::entity::location;
use crate::service::location::LocationService;

#[derive(Deserialize)]
pub struct CreateLocationRequest {
    pub warehouse_id: Uuid,
    pub location_no: String,
    pub location_name: String,
    pub location_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateLocationRequest {
    pub location_name: Option<String>,
    pub location_type: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct LocationResponse {
    pub id: Uuid,
    pub warehouse_id: Uuid,
    pub location_no: String,
    pub location_name: String,
    pub location_type: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<location::Model> for LocationResponse {
    fn from(model: location::Model) -> Self {
        Self {
            id: model.id,
            warehouse_id: model.warehouse_id,
            location_no: model.location_no,
            location_name: model.location_name,
            location_type: model.location_type,
            description: model.description,
            status: model.status,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

pub fn create_location_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/locations", post(create_location))
        .route("/locations", get(list_locations))
        .route("/locations/:id", get(get_location))
        .route("/locations/:id", put(update_location))
        .route("/locations/:id", delete(delete_location))
        .route("/warehouses/:warehouse_id/locations", get(list_by_warehouse))
        .with_state(db)
}

async fn create_location(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateLocationRequest>,
) -> Result<Json<ApiResponse<LocationResponse>>, StatusCode> {
    let service = LocationService::new(db);

    let location = service
        .create(
            tenant_id,
            req.warehouse_id,
            req.location_no,
            req.location_name,
            req.location_type,
            req.description,
        )
        .await
        .map_err(|e| {
            eprintln!("Error creating location: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(location.into()),
        message: "库位创建成功".to_string(),
    }))
}

async fn get_location(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<LocationResponse>>, StatusCode> {
    let service = LocationService::new(db);

    let location = service
        .get_by_id(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(location.into()),
        message: "查询成功".to_string(),
    }))
}

async fn list_locations(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<LocationResponse>>>, StatusCode> {
    let service = LocationService::new(db);

    let locations = service
        .list_all(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(locations.into_iter().map(|l| l.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn update_location(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<UpdateLocationRequest>,
) -> Result<Json<ApiResponse<LocationResponse>>, StatusCode> {
    let service = LocationService::new(db);

    let location = service
        .update(
            tenant_id,
            id,
            req.location_name,
            req.location_type,
            req.description,
            req.status,
        )
        .await
        .map_err(|e| {
            eprintln!("Error updating location: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(location.into()),
        message: "库位更新成功".to_string(),
    }))
}

async fn delete_location(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = LocationService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "库位删除成功".to_string(),
    }))
}

async fn list_by_warehouse(
    Path((tenant_id, warehouse_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<LocationResponse>>>, StatusCode> {
    let service = LocationService::new(db);

    let locations = service
        .list_by_warehouse(tenant_id, warehouse_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(locations.into_iter().map(|l| l.into()).collect()),
        message: "查询成功".to_string(),
    }))
}
