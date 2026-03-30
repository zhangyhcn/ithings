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

use crate::entity::schedule_plan;
use crate::service::schedule_plan::SchedulePlanService;

#[derive(Deserialize)]
pub struct CreateSchedulePlanRequest {
    pub work_order_id: Uuid,
    pub process_id: Uuid,
    pub equipment_id: Option<Uuid>,
    pub operator_id: Option<Uuid>,
    pub plan_quantity: Decimal,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
}



#[derive(Serialize)]
pub struct SchedulePlanResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub plan_no: String,
    pub work_order_id: Uuid,
    pub process_id: Uuid,
    pub equipment_id: Option<Uuid>,
    pub operator_id: Option<Uuid>,
    pub plan_quantity: Decimal,

    pub status: String,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<schedule_plan::Model> for SchedulePlanResponse {
    fn from(model: schedule_plan::Model) -> Self {
        Self {
            id: model.id,
            tenant_id: model.tenant_id,
            plan_no: model.plan_no,
            work_order_id: model.work_order_id,
            process_id: model.process_id,
            equipment_id: model.equipment_id,
            operator_id: model.operator_id,
            plan_quantity: model.plan_quantity,
            status: model.status,
            start_time: model.start_time,
            end_time: model.end_time,
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

pub fn create_schedule_plan_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/schedule-plans", post(create_schedule_plan))
        .route("/schedule-plans", get(list_schedule_plans))
        .route("/schedule-plans/:id", get(get_schedule_plan))
        .route("/schedule-plans/:id", delete(delete_schedule_plan))
        .route("/schedule-plans/:id/start", post(start_schedule_plan))
        .route("/schedule-plans/:id/complete", post(complete_schedule_plan))
        .route("/work-orders/:work_order_id/schedule-plans", get(list_by_work_order))
        .with_state(db)
}

async fn create_schedule_plan(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateSchedulePlanRequest>,
) -> Result<Json<ApiResponse<SchedulePlanResponse>>, StatusCode> {
    let service = SchedulePlanService::new(db);

    let plan = service
        .create(
            tenant_id,
            req.work_order_id,
            req.process_id,
            req.equipment_id,
            req.operator_id,
            req.plan_quantity,
            req.start_time,
            req.end_time,
        )
        .await
        .map_err(|e| {
            eprintln!("Error creating schedule plan: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plan.into()),
        message: "排程创建成功".to_string(),
    }))
}

async fn get_schedule_plan(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<SchedulePlanResponse>>, StatusCode> {
    let service = SchedulePlanService::new(db);

    let plan = service
        .get_by_id(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plan.into()),
        message: "查询成功".to_string(),
    }))
}

async fn list_schedule_plans(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<SchedulePlanResponse>>>, StatusCode> {
    let service = SchedulePlanService::new(db);

    let plans = service
        .list_all(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plans.into_iter().map(|p| p.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn delete_schedule_plan(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = SchedulePlanService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "排程删除成功".to_string(),
    }))
}

async fn start_schedule_plan(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<SchedulePlanResponse>>, StatusCode> {
    let service = SchedulePlanService::new(db);

    let plan = service
        .start(tenant_id, id)
        .await
        .map_err(|e| {
            eprintln!("Error starting schedule plan: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plan.into()),
        message: "排程开始成功".to_string(),
    }))
}

async fn complete_schedule_plan(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<SchedulePlanResponse>>, StatusCode> {
    let service = SchedulePlanService::new(db);

    let plan = service
        .complete(tenant_id, id)
        .await
        .map_err(|e| {
            eprintln!("Error completing schedule plan: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plan.into()),
        message: "排程完成成功".to_string(),
    }))
}

async fn list_by_work_order(
    Path((tenant_id, work_order_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<SchedulePlanResponse>>>, StatusCode> {
    let service = SchedulePlanService::new(db);

    let plans = service
        .list_by_work_order(tenant_id, work_order_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plans.into_iter().map(|p| p.into()).collect()),
        message: "查询成功".to_string(),
    }))
}
