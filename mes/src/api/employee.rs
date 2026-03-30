use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    response::Response,
    service::employee::{EmployeeService, CreateEmployeeRequest, EmployeeResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEmployeeRequest {
    pub name: Option<String>,
    pub department_id: Option<Uuid>,
    pub position: Option<String>,
    pub phone: Option<String>,
    pub entry_date: Option<String>,
    pub status: Option<String>,
}

pub fn create_employee_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/employees", get(list_employees).post(create_employee))
        .route("/employees/:id", get(get_employee).put(update_employee).delete(delete_employee))
        .with_state(db)
}

async fn list_employees(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<ResponseJson<Response<Vec<EmployeeResponse>>>, AppError> {
    let service = EmployeeService::new(db);
    let employees = service.list_all(tenant_id).await?;
    Ok(ResponseJson(Response::success(employees)))
}

async fn create_employee(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateEmployeeRequest>,
) -> Result<ResponseJson<Response<EmployeeResponse>>, AppError> {
    let service = EmployeeService::new(db);
    let employee = service.create(tenant_id, req).await?;
    Ok(ResponseJson(Response::success(employee)))
}

async fn get_employee(
    State(db): State<DatabaseConnection>,
    Path((TenantPath { tenant_id }, id)): Path<(TenantPath, Uuid)>,
) -> Result<ResponseJson<Response<EmployeeResponse>>, AppError> {
    let service = EmployeeService::new(db);
    let employee = service.get_by_id(tenant_id, id).await?
        .ok_or_else(|| AppError::not_found("Employee not found".to_string()))?;
    Ok(ResponseJson(Response::success(employee)))
}

async fn update_employee(
    State(db): State<DatabaseConnection>,
    Path((TenantPath { tenant_id }, id)): Path<(TenantPath, Uuid)>,
    Json(req): Json<UpdateEmployeeRequest>,
) -> Result<ResponseJson<Response<EmployeeResponse>>, AppError> {
    let service = EmployeeService::new(db);
    let employee = service.update(
        tenant_id,
        id,
        req.name,
        req.department_id,
        req.position,
        req.phone,
        req.entry_date,
        req.status,
    ).await?;
    Ok(ResponseJson(Response::success(employee)))
}

async fn delete_employee(
    State(db): State<DatabaseConnection>,
    Path((TenantPath { tenant_id }, id)): Path<(TenantPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = EmployeeService::new(db);
    service.delete(tenant_id, id).await?;
    Ok(ResponseJson(Response::success(())))
}
