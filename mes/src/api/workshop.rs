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

use crate::entity::workshop;
use crate::service::workshop::WorkshopService;

#[derive(Deserialize)]
pub struct CreateWorkshopRequest {
    pub workshop_no: String,
    pub workshop_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateWorkshopRequest {
    pub workshop_name: Option<String>,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct WorkshopResponse {
    pub id: Uuid,
    pub workshop_no: String,
    pub workshop_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<workshop::Model> for WorkshopResponse {
    fn from(model: workshop::Model) -> Self {
        Self {
            id: model.id,
            workshop_no: model.workshop_no,
            workshop_name: model.workshop_name,
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

pub fn create_workshop_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/workshops", post(create_workshop))
        .route("/workshops", get(list_workshops))
        .route("/workshops/:id", get(get_workshop))
        .route("/workshops/:id", put(update_workshop))
        .route("/workshops/:id", delete(delete_workshop))
        .with_state(db)
}

async fn create_workshop(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateWorkshopRequest>,
) -> Result<Json<ApiResponse<WorkshopResponse>>, StatusCode> {
    let service = WorkshopService::new(db);

    let workshop = service
        .create(tenant_id, req.workshop_no, req.workshop_name, req.location, req.description)
        .await
        .map_err(|e| {
            eprintln!("Error creating workshop: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(workshop.into()),
        message: "车间创建成功".to_string(),
    }))
}

async fn get_workshop(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<WorkshopResponse>>, StatusCode> {
    let service = WorkshopService::new(db);

    let workshop = service
        .get_by_id(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(workshop.into()),
        message: "查询成功".to_string(),
    }))
}

async fn list_workshops(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<WorkshopResponse>>>, StatusCode> {
    let service = WorkshopService::new(db);

    let workshops = service
        .list_all(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(workshops.into_iter().map(|w| w.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn update_workshop(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<UpdateWorkshopRequest>,
) -> Result<Json<ApiResponse<WorkshopResponse>>, StatusCode> {
    let service = WorkshopService::new(db);

    let workshop = service
        .update(tenant_id, id, req.workshop_name, req.location, req.description, req.status)
        .await
        .map_err(|e| {
            eprintln!("Error updating workshop: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(workshop.into()),
        message: "车间更新成功".to_string(),
    }))
}

async fn delete_workshop(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = WorkshopService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "车间删除成功".to_string(),
    }))
}
