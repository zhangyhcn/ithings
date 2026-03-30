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
    service::stocktaking::{StocktakingService, CreateStocktakingRequest, UpdateStocktakingRequest, StocktakingResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_stocktaking_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/stocktakings", get(list_stocktakings).post(create_stocktaking))
        .route("/stocktakings/:id", get(get_stocktaking).put(update_stocktaking).delete(delete_stocktaking))
        .with_state(db)
}

async fn list_stocktakings(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<StocktakingResponse>>>, AppError> {
    let service = StocktakingService::new(db);
    let stocktakings = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(stocktakings)))
}

async fn create_stocktaking(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreateStocktakingRequest>,
) -> Result<ResponseJson<Response<StocktakingResponse>>, AppError> {
    let service = StocktakingService::new(db);
    let stocktaking = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(stocktaking)))
}

async fn get_stocktaking(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<StocktakingResponse>>, AppError> {
    let service = StocktakingService::new(db);
    let stocktaking = service.get(id).await?;
    Ok(ResponseJson(Response::success(stocktaking)))
}

async fn update_stocktaking(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
    Json(req): Json<UpdateStocktakingRequest>,
) -> Result<ResponseJson<Response<StocktakingResponse>>, AppError> {
    let service = StocktakingService::new(db);
    let stocktaking = service.update(id, req).await?;
    Ok(ResponseJson(Response::success(stocktaking)))
}

async fn delete_stocktaking(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = StocktakingService::new(db);
    service.delete(id).await?;
    Ok(ResponseJson(Response::success(())))
}
