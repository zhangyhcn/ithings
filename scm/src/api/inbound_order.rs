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
    service::inbound_order::{InboundOrderService, CreateInboundOrderRequest, InboundOrderResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_inbound_order_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/inbound-orders", get(list_inbound_orders).post(create_inbound_order))
        .route("/inbound-orders/:id", delete(delete_inbound_order))
        .with_state(db)
}

async fn list_inbound_orders(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<InboundOrderResponse>>>, AppError> {
    let service = InboundOrderService::new(db);
    let orders = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(orders)))
}

async fn create_inbound_order(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreateInboundOrderRequest>,
) -> Result<ResponseJson<Response<InboundOrderResponse>>, AppError> {
    let service = InboundOrderService::new(db);
    let order = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(order)))
}

async fn delete_inbound_order(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = InboundOrderService::new(db);
    service.delete(tenant_id, org_id, id).await?;
    Ok(ResponseJson(Response::success(())))
}
