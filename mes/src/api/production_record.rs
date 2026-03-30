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

use crate::entity::production_record;
use crate::service::production_record::ProductionRecordService;

#[derive(Deserialize)]
pub struct CreateProductionRecordRequest {
    pub work_order_id: Uuid,
    pub process_id: Uuid,
    pub equipment_id: Option<Uuid>,
    pub operator_id: Option<Uuid>,
    pub batch_no: Option<String>,
    pub quantity: Decimal,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub process_data: Option<JsonValue>,
}

#[derive(Deserialize)]
pub struct UpdateProductionRecordRequest {
    pub good_qty: Option<Decimal>,
    pub defect_qty: Option<Decimal>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub process_data: Option<JsonValue>,
}

#[derive(Serialize)]
pub struct ProductionRecordResponse {
    pub id: Uuid,
    pub work_order_id: Uuid,
    pub process_id: Uuid,
    pub equipment_id: Option<Uuid>,
    pub operator_id: Option<Uuid>,
    pub batch_no: Option<String>,
    pub quantity: Decimal,
    pub good_qty: Option<Decimal>,
    pub defect_qty: Option<Decimal>,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub process_data: Option<JsonValue>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<production_record::Model> for ProductionRecordResponse {
    fn from(model: production_record::Model) -> Self {
        Self {
            id: model.id,
            work_order_id: model.work_order_id,
            process_id: model.process_id,
            equipment_id: model.equipment_id,
            operator_id: model.operator_id,
            batch_no: model.batch_no,
            quantity: model.quantity,
            good_qty: model.good_qty,
            defect_qty: model.defect_qty,
            start_time: model.start_time,
            end_time: model.end_time,
            process_data: model.process_data,
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

pub fn create_production_record_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/production-records", post(create_production_record))
        .route("/production-records", get(list_production_records))
        .route("/production-records/:id", get(get_production_record))
        .route("/production-records/:id", put(update_production_record))
        .route("/production-records/:id", delete(delete_production_record))
        .route("/work-orders/:work_order_id/production-records", get(list_by_work_order))
        .route("/processes/:process_id/production-records", get(list_by_process))
        .with_state(db)
}

async fn create_production_record(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateProductionRecordRequest>,
) -> Result<Json<ApiResponse<ProductionRecordResponse>>, StatusCode> {
    let service = ProductionRecordService::new(db);
    
    let start_time = req.start_time.map(|t| t.naive_utc());
    
    let record = service
        .create(
            tenant_id,
            req.work_order_id,
            req.process_id,
            req.equipment_id,
            req.operator_id,
            req.batch_no,
            req.quantity,
            start_time,
            req.process_data,
        )
        .await
        .map_err(|e| {
            eprintln!("Error creating production record: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(record.into()),
        message: "生产记录创建成功".to_string(),
    }))
}

async fn get_production_record(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<ProductionRecordResponse>>, StatusCode> {
    let service = ProductionRecordService::new(db);

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

async fn list_production_records(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<ProductionRecordResponse>>>, StatusCode> {
    let service = ProductionRecordService::new(db);

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

async fn update_production_record(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<UpdateProductionRecordRequest>,
) -> Result<Json<ApiResponse<ProductionRecordResponse>>, StatusCode> {
    let service = ProductionRecordService::new(db);
    
    let end_time = req.end_time.map(|t| t.naive_utc());
    
    let record = service
        .update(
            tenant_id,
            id,
            req.good_qty,
            req.defect_qty,
            end_time,
            req.process_data,
        )
        .await
        .map_err(|e| {
            eprintln!("Error updating production record: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(record.into()),
        message: "生产记录更新成功".to_string(),
    }))
}

async fn delete_production_record(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = ProductionRecordService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "生产记录删除成功".to_string(),
    }))
}

async fn list_by_work_order(
    Path((tenant_id, work_order_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<ProductionRecordResponse>>>, StatusCode> {
    let service = ProductionRecordService::new(db);

    let records = service
        .list_by_work_order(tenant_id, work_order_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(records.into_iter().map(|r| r.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn list_by_process(
    Path((tenant_id, process_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<ProductionRecordResponse>>>, StatusCode> {
    let service = ProductionRecordService::new(db);

    let records = service
        .list_by_process(tenant_id, process_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(records.into_iter().map(|r| r.into()).collect()),
        message: "查询成功".to_string(),
    }))
}
