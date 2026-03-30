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

use crate::entity::warehouse;
use crate::service::warehouse::WarehouseService;

#[derive(Deserialize)]
pub struct CreateWarehouseRequest {
    pub warehouse_no: String,
    pub warehouse_name: String,
    pub warehouse_type: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateWarehouseRequest {
    pub warehouse_name: Option<String>,
    pub warehouse_type: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct WarehouseResponse {
    pub id: Uuid,
    pub warehouse_no: String,
    pub warehouse_name: String,
    pub warehouse_type: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<warehouse::Model> for WarehouseResponse {
    fn from(model: warehouse::Model) -> Self {
        Self {
            id: model.id,
            warehouse_no: model.warehouse_no,
            warehouse_name: model.warehouse_name,
            warehouse_type: model.warehouse_type,
            location: model.location,
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

pub fn create_warehouse_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/warehouses", post(create_warehouse))
        .route("/warehouses", get(list_warehouses))
        .route("/warehouses/:id", get(get_warehouse))
        .route("/warehouses/:id", put(update_warehouse))
        .route("/warehouses/:id", delete(delete_warehouse))
        .with_state(db)
}

async fn create_warehouse(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateWarehouseRequest>,
) -> Result<Json<ApiResponse<WarehouseResponse>>, StatusCode> {
    let service = WarehouseService::new(db);

    let warehouse = service
        .create(
            tenant_id,
            req.warehouse_no,
            req.warehouse_name,
            req.warehouse_type,
            req.location,
            req.description,
        )
        .await
        .map_err(|e| {
            eprintln!("Error creating warehouse: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(warehouse.into()),
        message: "仓库创建成功".to_string(),
    }))
}

async fn get_warehouse(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<WarehouseResponse>>, StatusCode> {
    let service = WarehouseService::new(db);

    let warehouse = service
        .get_by_id(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(warehouse.into()),
        message: "查询成功".to_string(),
    }))
}

async fn list_warehouses(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<WarehouseResponse>>>, StatusCode> {
    let service = WarehouseService::new(db);

    let warehouses = service
        .list_all(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(warehouses.into_iter().map(|w| w.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn update_warehouse(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<UpdateWarehouseRequest>,
) -> Result<Json<ApiResponse<WarehouseResponse>>, StatusCode> {
    let service = WarehouseService::new(db);

    let warehouse = service
        .update(
            tenant_id,
            id,
            req.warehouse_name,
            req.warehouse_type,
            req.location,
            req.description,
            req.status,
        )
        .await
        .map_err(|e| {
            eprintln!("Error updating warehouse: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(warehouse.into()),
        message: "仓库更新成功".to_string(),
    }))
}

async fn delete_warehouse(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = WarehouseService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "仓库删除成功".to_string(),
    }))
}
