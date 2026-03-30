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

use crate::entity::maintenance_plan;
use crate::service::maintenance_plan::MaintenancePlanService;

#[derive(Deserialize)]
pub struct CreateMaintenancePlanRequest {
    pub equipment_id: Uuid,
    pub plan_type: String,
    pub plan_date: Option<chrono::NaiveDate>,
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct ExecuteMaintenancePlanRequest {
    pub executor_id: Uuid,
    pub content: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

#[derive(Serialize)]
pub struct MaintenancePlanResponse {
    pub id: Uuid,
    pub equipment_id: Uuid,
    pub plan_type: String,
    pub plan_date: Option<chrono::NaiveDate>,
    pub content: Option<String>,
    pub status: String,
    pub executor_id: Option<Uuid>,
    pub execute_time: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<maintenance_plan::Model> for MaintenancePlanResponse {
    fn from(model: maintenance_plan::Model) -> Self {
        Self {
            id: model.id,
            equipment_id: model.equipment_id,
            plan_type: model.plan_type,
            plan_date: model.plan_date,
            content: model.content,
            status: model.status,
            executor_id: model.executor_id,
            execute_time: model.execute_time,
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

pub fn create_maintenance_plan_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/maintenance-plans", post(create_maintenance_plan))
        .route("/maintenance-plans", get(list_maintenance_plans))
        .route("/maintenance-plans/:id", get(get_maintenance_plan))
        .route("/maintenance-plans/:id", put(update_maintenance_plan_status))
        .route("/maintenance-plans/:id", delete(delete_maintenance_plan))
        .route("/maintenance-plans/:id/execute", post(execute_maintenance_plan))
        .route("/equipment/:equipment_id/maintenance-plans", get(list_by_equipment))
        .route("/maintenance-plans/status/:status", get(list_by_status))
        .with_state(db)
}

async fn create_maintenance_plan(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateMaintenancePlanRequest>,
) -> Result<Json<ApiResponse<MaintenancePlanResponse>>, StatusCode> {
    let service = MaintenancePlanService::new(db);

    let plan = service
        .create(
            tenant_id,
            req.equipment_id,
            req.plan_type,
            req.plan_date,
            req.content,
        )
        .await
        .map_err(|e| {
            eprintln!("Error creating maintenance plan: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plan.into()),
        message: "维护计划创建成功".to_string(),
    }))
}

async fn get_maintenance_plan(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<MaintenancePlanResponse>>, StatusCode> {
    let service = MaintenancePlanService::new(db);

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

async fn list_maintenance_plans(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<MaintenancePlanResponse>>>, StatusCode> {
    let service = MaintenancePlanService::new(db);

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

async fn update_maintenance_plan_status(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<UpdateStatusRequest>,
) -> Result<Json<ApiResponse<MaintenancePlanResponse>>, StatusCode> {
    let service = MaintenancePlanService::new(db);

    let plan = service
        .update_status(tenant_id, id, req.status)
        .await
        .map_err(|e| {
            eprintln!("Error updating maintenance plan: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plan.into()),
        message: "维护计划状态更新成功".to_string(),
    }))
}

async fn execute_maintenance_plan(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<ExecuteMaintenancePlanRequest>,
) -> Result<Json<ApiResponse<MaintenancePlanResponse>>, StatusCode> {
    let service = MaintenancePlanService::new(db);

    let plan = service
        .execute(tenant_id, id, req.executor_id, req.content)
        .await
        .map_err(|e| {
            eprintln!("Error executing maintenance plan: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plan.into()),
        message: "维护计划执行成功".to_string(),
    }))
}

async fn delete_maintenance_plan(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = MaintenancePlanService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "维护计划删除成功".to_string(),
    }))
}

async fn list_by_equipment(
    Path((tenant_id, equipment_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<MaintenancePlanResponse>>>, StatusCode> {
    let service = MaintenancePlanService::new(db);

    let plans = service
        .list_by_equipment(tenant_id, equipment_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plans.into_iter().map(|p| p.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn list_by_status(
    Path((tenant_id, status)): Path<(Uuid, String)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<MaintenancePlanResponse>>>, StatusCode> {
    let service = MaintenancePlanService::new(db);

    let plans = service
        .list_by_status(tenant_id, status)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(plans.into_iter().map(|p| p.into()).collect()),
        message: "查询成功".to_string(),
    }))
}
