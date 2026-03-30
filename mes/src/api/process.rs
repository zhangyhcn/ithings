use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    Router,
    routing::{get, post, put, delete},
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use uuid::Uuid;

use crate::entity::process;
use crate::service::process::ProcessService;

#[derive(Deserialize)]
pub struct CreateProcessRequest {
    pub route_id: Uuid,
    pub process_no: String,
    pub process_name: String,
    pub sequence: i32,
    pub work_station_id: Option<Uuid>,
    pub standard_time: Option<Decimal>,
    pub setup_time: Option<Decimal>,
    pub process_params: Option<JsonValue>,
    pub next_process_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateProcessRequest {
    pub process_name: Option<String>,
    pub sequence: Option<i32>,
    pub work_station_id: Option<Uuid>,
    pub standard_time: Option<Decimal>,
    pub setup_time: Option<Decimal>,
    pub process_params: Option<JsonValue>,
    pub next_process_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct ProcessResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub route_id: Uuid,
    pub process_no: String,
    pub process_name: String,
    pub sequence: i32,
    pub work_station_id: Option<Uuid>,
    pub standard_time: Option<Decimal>,
    pub setup_time: Option<Decimal>,
    pub process_params: Option<JsonValue>,
    pub next_process_id: Option<Uuid>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<process::Model> for ProcessResponse {
    fn from(model: process::Model) -> Self {
        Self {
            id: model.id,
            tenant_id: model.tenant_id,
            route_id: model.route_id,
            process_no: model.process_no,
            process_name: model.process_name,
            sequence: model.sequence,
            work_station_id: model.work_station_id,
            standard_time: model.standard_time,
            setup_time: model.setup_time,
            process_params: model.process_params,
            next_process_id: model.next_process_id,
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

pub fn create_process_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/processes", post(create_process))
        .route("/processes", get(list_processes))
        .route("/processes/:id", get(get_process))
        .route("/processes/:id", put(update_process))
        .route("/processes/:id", delete(delete_process))
        .route("/process-routes/:route_id/processes", get(list_by_route))
        .with_state(db)
}

async fn create_process(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateProcessRequest>,
) -> Result<Json<ApiResponse<ProcessResponse>>, StatusCode> {
    let service = ProcessService::new(db);

    let process = service
        .create(
            tenant_id,
            req.route_id,
            req.process_no,
            req.process_name,
            req.sequence,
            req.work_station_id,
            req.standard_time,
            req.setup_time,
            req.process_params,
            req.next_process_id,
        )
        .await
        .map_err(|e| {
            eprintln!("Error creating process: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(process.into()),
        message: "工序创建成功".to_string(),
    }))
}

async fn get_process(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<ProcessResponse>>, StatusCode> {
    let service = ProcessService::new(db);

    let process = service
        .get_by_id(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(process.into()),
        message: "查询成功".to_string(),
    }))
}

async fn list_processes(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<ProcessResponse>>>, StatusCode> {
    let service = ProcessService::new(db);

    let processes = service
        .list_all(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(processes.into_iter().map(|p| p.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn update_process(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<UpdateProcessRequest>,
) -> Result<Json<ApiResponse<ProcessResponse>>, StatusCode> {
    let service = ProcessService::new(db);

    let process = service
        .update(
            tenant_id,
            id,
            req.process_name,
            req.sequence,
            req.work_station_id,
            req.standard_time,
            req.setup_time,
            req.process_params,
            req.next_process_id,
        )
        .await
        .map_err(|e| {
            eprintln!("Error updating process: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(process.into()),
        message: "工序更新成功".to_string(),
    }))
}

async fn delete_process(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = ProcessService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "工序删除成功".to_string(),
    }))
}

async fn list_by_route(
    Path((tenant_id, route_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<ProcessResponse>>>, StatusCode> {
    let service = ProcessService::new(db);

    let processes = service
        .list_by_route(tenant_id, route_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(processes.into_iter().map(|p| p.into()).collect()),
        message: "查询成功".to_string(),
    }))
}
