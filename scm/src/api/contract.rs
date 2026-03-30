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
    service::contract::{ContractService, CreateContractRequest, UpdateContractRequest, ContractResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_contract_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/contracts", get(list_contracts).post(create_contract))
        .route("/contracts/:id", get(get_contract).put(update_contract).delete(delete_contract))
        .with_state(db)
}

async fn list_contracts(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<ContractResponse>>>, AppError> {
    let service = ContractService::new(db);
    let contracts = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(contracts)))
}

async fn create_contract(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreateContractRequest>,
) -> Result<ResponseJson<Response<ContractResponse>>, AppError> {
    let service = ContractService::new(db);
    let contract = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(contract)))
}

async fn get_contract(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<ContractResponse>>, AppError> {
    let service = ContractService::new(db);
    let contract = service.get(id).await?;
    Ok(ResponseJson(Response::success(contract)))
}

async fn update_contract(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
    Json(req): Json<UpdateContractRequest>,
) -> Result<ResponseJson<Response<ContractResponse>>, AppError> {
    let service = ContractService::new(db);
    let contract = service.update(id, req).await?;
    Ok(ResponseJson(Response::success(contract)))
}

async fn delete_contract(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = ContractService::new(db);
    service.delete(id).await?;
    Ok(ResponseJson(Response::success(())))
}
