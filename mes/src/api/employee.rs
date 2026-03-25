use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
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

pub fn create_employee_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/employees", get(list_employees).post(create_employee))
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
