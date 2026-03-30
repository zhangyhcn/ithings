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

use crate::entity::defect_record;
use crate::service::defect_record::DefectRecordService;

#[derive(Deserialize)]
pub struct CreateDefectRecordRequest {
    pub inspection_id: Uuid,
    pub quantity: i32,
    pub defect_type_id: Option<Uuid>,
    pub defect_code: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct HandleDefectRequest {
    pub disposition: String,
}

#[derive(Deserialize)]
pub struct UpdateDefectRecordRequest {
    pub quantity: Option<i32>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct DefectRecordResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inspection_id: Uuid,
    pub defect_type_id: Option<Uuid>,
    pub defect_code: Option<String>,
    pub quantity: i32,
    pub description: Option<String>,
    pub disposition: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<defect_record::Model> for DefectRecordResponse {
    fn from(model: defect_record::Model) -> Self {
        Self {
            id: model.id,
            tenant_id: model.tenant_id,
            inspection_id: model.inspection_id,
            defect_type_id: model.defect_type_id,
            defect_code: model.defect_code,
            quantity: model.quantity,
            description: model.description,
            disposition: model.disposition,
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

pub fn create_defect_record_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/defect-records", post(create_defect_record))
        .route("/defect-records", get(list_defect_records))
        .route("/defect-records/:id", get(get_defect_record))
        .route("/defect-records/:id", put(update_defect_record))
        .route("/defect-records/:id", delete(delete_defect_record))
        .route("/defect-records/:id/handle", post(handle_defect))
        .route("/inspections/:inspection_id/defect-records", get(list_by_inspection))
        .with_state(db)
}

async fn create_defect_record(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateDefectRecordRequest>,
) -> Result<Json<ApiResponse<DefectRecordResponse>>, StatusCode> {
    let service = DefectRecordService::new(db);

    let record = service
        .create(
            tenant_id,
            req.inspection_id,
            req.quantity,
            req.defect_type_id,
            req.defect_code,
            req.description,
        )
        .await
        .map_err(|e| {
            eprintln!("Error creating defect record: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(record.into()),
        message: "不良记录创建成功".to_string(),
    }))
}

async fn get_defect_record(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<DefectRecordResponse>>, StatusCode> {
    let service = DefectRecordService::new(db);

    let record = service
        .get_by_id(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(record.into()),
        message: "查询成功".to_string(),
    }))
}

async fn list_defect_records(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<DefectRecordResponse>>>, StatusCode> {
    let service = DefectRecordService::new(db);

    let records = service
        .list_all(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(records.into_iter().map(|r| r.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn update_defect_record(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<UpdateDefectRecordRequest>,
) -> Result<Json<ApiResponse<DefectRecordResponse>>, StatusCode> {
    let service = DefectRecordService::new(db);

    let record = service
        .update(
            tenant_id,
            id,
            req.quantity,
            req.description,
            req.status,
        )
        .await
        .map_err(|e| {
            eprintln!("Error updating defect record: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(record.into()),
        message: "不良记录更新成功".to_string(),
    }))
}

async fn delete_defect_record(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = DefectRecordService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "不良记录删除成功".to_string(),
    }))
}

async fn handle_defect(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<HandleDefectRequest>,
) -> Result<Json<ApiResponse<DefectRecordResponse>>, StatusCode> {
    let service = DefectRecordService::new(db);

    let record = service
        .handle(tenant_id, id, req.disposition)
        .await
        .map_err(|e| {
            eprintln!("Error handling defect: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(record.into()),
        message: "不良品处理成功".to_string(),
    }))
}

async fn list_by_inspection(
    Path((tenant_id, inspection_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<DefectRecordResponse>>>, StatusCode> {
    let service = DefectRecordService::new(db);

    let records = service
        .list_by_inspection(tenant_id, inspection_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(records.into_iter().map(|r| r.into()).collect()),
        message: "查询成功".to_string(),
    }))
}
