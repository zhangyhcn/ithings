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

use crate::entity::production_line;
use crate::service::production_line::ProductionLineService;

#[derive(Deserialize)]
pub struct CreateProductionLineRequest {
    pub workshop_id: Option<Uuid>,
    pub line_no: String,
    pub line_name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateProductionLineRequest {
    pub workshop_id: Option<Uuid>,
    pub line_name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct ProductionLineResponse {
    pub id: Uuid,
    pub workshop_id: Option<Uuid>,
    pub line_no: String,
    pub line_name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<production_line::Model> for ProductionLineResponse {
    fn from(model: production_line::Model) -> Self {
        Self {
            id: model.id,
            workshop_id: model.workshop_id,
            line_no: model.line_no,
            line_name: model.line_name,
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

pub fn create_production_line_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/production-lines", post(create_production_line))
        .route("/production-lines", get(list_production_lines))
        .route("/production-lines/:id", get(get_production_line))
        .route("/production-lines/:id", put(update_production_line))
        .route("/production-lines/:id", delete(delete_production_line))
        .route("/workshops/:workshop_id/production-lines", get(list_by_workshop))
        .with_state(db)
}

async fn create_production_line(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateProductionLineRequest>,
) -> Result<Json<ApiResponse<ProductionLineResponse>>, StatusCode> {
    let service = ProductionLineService::new(db);

    let line = service
        .create(tenant_id, req.workshop_id, req.line_no, req.line_name, req.description)
        .await
        .map_err(|e| {
            eprintln!("Error creating production line: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(line.into()),
        message: "产线创建成功".to_string(),
    }))
}

async fn get_production_line(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<ProductionLineResponse>>, StatusCode> {
    let service = ProductionLineService::new(db);

    let line = service
        .get_by_id(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(line.into()),
        message: "查询成功".to_string(),
    }))
}

async fn list_production_lines(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<ProductionLineResponse>>>, StatusCode> {
    let service = ProductionLineService::new(db);

    let lines = service
        .list_all(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(lines.into_iter().map(|l| l.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn update_production_line(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<UpdateProductionLineRequest>,
) -> Result<Json<ApiResponse<ProductionLineResponse>>, StatusCode> {
    let service = ProductionLineService::new(db);

    let line = service
        .update(tenant_id, id, req.workshop_id, req.line_name, req.description, req.status)
        .await
        .map_err(|e| {
            eprintln!("Error updating production line: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(line.into()),
        message: "产线更新成功".to_string(),
    }))
}

async fn delete_production_line(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = ProductionLineService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "产线删除成功".to_string(),
    }))
}

async fn list_by_workshop(
    Path((tenant_id, workshop_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<ProductionLineResponse>>>, StatusCode> {
    let service = ProductionLineService::new(db);

    let lines = service
        .list_by_workshop(tenant_id, workshop_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(lines.into_iter().map(|l| l.into()).collect()),
        message: "查询成功".to_string(),
    }))
}
