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
    service::supplier_quotation::{SupplierQuotationService, CreateSupplierQuotationRequest, UpdateSupplierQuotationRequest, SupplierQuotationResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_supplier_quotation_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/quotations", get(list_quotations).post(create_quotation))
        .route("/quotations/:id", get(get_quotation).put(update_quotation).delete(delete_quotation))
        .with_state(db)
}

async fn list_quotations(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<SupplierQuotationResponse>>>, AppError> {
    let service = SupplierQuotationService::new(db);
    let quotations = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(quotations)))
}

async fn create_quotation(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreateSupplierQuotationRequest>,
) -> Result<ResponseJson<Response<SupplierQuotationResponse>>, AppError> {
    let service = SupplierQuotationService::new(db);
    let quotation = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(quotation)))
}

async fn get_quotation(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<SupplierQuotationResponse>>, AppError> {
    let service = SupplierQuotationService::new(db);
    let quotation = service.get(id).await?;
    Ok(ResponseJson(Response::success(quotation)))
}

async fn update_quotation(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
    Json(req): Json<UpdateSupplierQuotationRequest>,
) -> Result<ResponseJson<Response<SupplierQuotationResponse>>, AppError> {
    let service = SupplierQuotationService::new(db);
    let quotation = service.update(id, req).await?;
    Ok(ResponseJson(Response::success(quotation)))
}

async fn delete_quotation(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = SupplierQuotationService::new(db);
    service.delete(id).await?;
    Ok(ResponseJson(Response::success(())))
}
