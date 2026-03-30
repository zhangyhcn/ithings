use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    response::Response,
    service::outbound_order::{OutboundOrderService, CreateOutboundOrderRequest, OutboundOrderResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_outbound_order_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/outbound-orders", get(list_outbound_orders).post(create_outbound_order))
        .route("/outbound-orders/:id", delete(delete_outbound_order))
        .with_state(db)
}

async fn list_outbound_orders(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<OutboundOrderResponse>>>, AppError> {
    let service = OutboundOrderService::new(db);
    let orders = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(orders)))
}

async fn create_outbound_order(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreateOutboundOrderRequest>,
) -> Result<ResponseJson<Response<OutboundOrderResponse>>, AppError> {
    let service = OutboundOrderService::new(db);
    let order = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(order)))
}

async fn delete_outbound_order(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = OutboundOrderService::new(db);
    service.delete(tenant_id, org_id, id).await?;
    Ok(ResponseJson(Response::success(())))
}
