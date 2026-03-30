use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    response::Response,
    service::supplier::{SupplierService, CreateSupplierRequest, UpdateSupplierRequest, SupplierResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_supplier_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/suppliers", get(list_suppliers).post(create_supplier))
        .route("/suppliers/:id", get(get_supplier).put(update_supplier).delete(delete_supplier))
        .with_state(db)
}

async fn list_suppliers(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<SupplierResponse>>>, AppError> {
    let service = SupplierService::new(db);
    let suppliers = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(suppliers)))
}

async fn create_supplier(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreateSupplierRequest>,
) -> Result<ResponseJson<Response<SupplierResponse>>, AppError> {
    let service = SupplierService::new(db);
    let supplier = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(supplier)))
}

async fn get_supplier(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<SupplierResponse>>, AppError> {
    let service = SupplierService::new(db);
    let supplier = service.get_by_id(tenant_id, org_id, id).await?
        .ok_or_else(|| AppError::not_found("Supplier not found".to_string()))?;
    Ok(ResponseJson(Response::success(supplier)))
}

async fn update_supplier(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
    Json(req): Json<UpdateSupplierRequest>,
) -> Result<ResponseJson<Response<SupplierResponse>>, AppError> {
    let service = SupplierService::new(db);
    let supplier = service.update(tenant_id, org_id, id, req).await?;
    Ok(ResponseJson(Response::success(supplier)))
}

async fn delete_supplier(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = SupplierService::new(db);
    service.delete(tenant_id, org_id, id).await?;
    Ok(ResponseJson(Response::success(())))
}
