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
    service::bidding::{BiddingService, CreateBiddingRequest, UpdateBiddingRequest, BiddingResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_bidding_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/biddings", get(list_biddings).post(create_bidding))
        .route("/biddings/:id", get(get_bidding).put(update_bidding).delete(delete_bidding))
        .with_state(db)
}

async fn list_biddings(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<BiddingResponse>>>, AppError> {
    let service = BiddingService::new(db);
    let biddings = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(biddings)))
}

async fn create_bidding(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreateBiddingRequest>,
) -> Result<ResponseJson<Response<BiddingResponse>>, AppError> {
    let service = BiddingService::new(db);
    let bidding = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(bidding)))
}

async fn get_bidding(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<BiddingResponse>>, AppError> {
    let service = BiddingService::new(db);
    let bidding = service.get(id).await?;
    Ok(ResponseJson(Response::success(bidding)))
}

async fn update_bidding(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
    Json(req): Json<UpdateBiddingRequest>,
) -> Result<ResponseJson<Response<BiddingResponse>>, AppError> {
    let service = BiddingService::new(db);
    let bidding = service.update(id, req).await?;
    Ok(ResponseJson(Response::success(bidding)))
}

async fn delete_bidding(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = BiddingService::new(db);
    service.delete(id).await?;
    Ok(ResponseJson(Response::success(())))
}
