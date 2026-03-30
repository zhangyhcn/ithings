use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    Router,
    routing::{get, post, put, delete},
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::entity::inspection_order;
use crate::service::inspection_order::InspectionOrderService;

#[derive(Deserialize)]
pub struct CreateInspectionOrderRequest {
    pub inspection_type: String,
    pub work_order_id: Option<Uuid>,
    pub material_id: Option<Uuid>,
    pub sample_qty: Option<i32>,
    pub inspector_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct SubmitResultRequest {
    pub pass_qty: i32,
    pub defect_qty: i32,
    pub result: String,
    pub inspector_id: Uuid,
}

#[derive(Serialize)]
pub struct InspectionOrderResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inspection_no: String,
    pub inspection_type: String,
    pub work_order_id: Option<Uuid>,
    pub material_id: Option<Uuid>,
    pub batch_no: Option<String>,
    pub sample_qty: Option<i32>,
    pub pass_qty: Option<i32>,
    pub defect_qty: Option<i32>,
    pub result: String,
    pub inspector_id: Option<Uuid>,
    pub inspect_time: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<inspection_order::Model> for InspectionOrderResponse {
    fn from(model: inspection_order::Model) -> Self {
        Self {
            id: model.id,
            tenant_id: model.tenant_id,
            inspection_no: model.inspection_no,
            inspection_type: model.inspection_type,
            work_order_id: model.work_order_id,
            material_id: model.material_id,
            batch_no: model.batch_no,
            sample_qty: model.sample_qty,
            pass_qty: model.pass_qty,
            defect_qty: model.defect_qty,
            result: model.result,
            inspector_id: model.inspector_id,
            inspect_time: model.inspect_time,
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

pub fn create_inspection_order_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/inspection-orders", post(create_inspection_order))
        .route("/inspection-orders", get(list_inspection_orders))
        .route("/inspection-orders/:id", get(get_inspection_order))
        .route("/inspection-orders/:id", delete(delete_inspection_order))
        .route("/inspection-orders/:id/submit", post(submit_result))
        .route("/inspection-orders/type/:inspection_type", get(list_by_type))
        .route("/work-orders/:work_order_id/inspection-orders", get(list_by_work_order))
        .with_state(db)
}

async fn create_inspection_order(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateInspectionOrderRequest>,
) -> Result<Json<ApiResponse<InspectionOrderResponse>>, StatusCode> {
    let service = InspectionOrderService::new(db);

    let order = service
        .create(
            tenant_id,
            req.inspection_type,
            req.work_order_id,
            req.material_id,
            req.sample_qty,
            req.inspector_id,
        )
        .await
        .map_err(|e| {
            eprintln!("Error creating inspection order: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(order.into()),
        message: "检验单创建成功".to_string(),
    }))
}

async fn get_inspection_order(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<InspectionOrderResponse>>, StatusCode> {
    let service = InspectionOrderService::new(db);

    let order = service
        .get_by_id(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(order.into()),
        message: "查询成功".to_string(),
    }))
}

async fn list_inspection_orders(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<InspectionOrderResponse>>>, StatusCode> {
    let service = InspectionOrderService::new(db);

    let orders = service
        .list_all(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(orders.into_iter().map(|o| o.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn delete_inspection_order(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = InspectionOrderService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "检验单删除成功".to_string(),
    }))
}

async fn submit_result(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<SubmitResultRequest>,
) -> Result<Json<ApiResponse<InspectionOrderResponse>>, StatusCode> {
    let service = InspectionOrderService::new(db);

    let order = service
        .submit_result(tenant_id, id, req.pass_qty, req.defect_qty, req.result, req.inspector_id)
        .await
        .map_err(|e| {
            eprintln!("Error submitting result: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(order.into()),
        message: "检验结果提交成功".to_string(),
    }))
}

async fn list_by_type(
    Path((tenant_id, inspection_type)): Path<(Uuid, String)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<InspectionOrderResponse>>>, StatusCode> {
    let service = InspectionOrderService::new(db);

    let orders = service
        .list_by_type(tenant_id, inspection_type)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(orders.into_iter().map(|o| o.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn list_by_work_order(
    Path((tenant_id, work_order_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<InspectionOrderResponse>>>, StatusCode> {
    let service = InspectionOrderService::new(db);

    let orders = service
        .list_by_work_order(tenant_id, work_order_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(orders.into_iter().map(|o| o.into()).collect()),
        message: "查询成功".to_string(),
    }))
}
