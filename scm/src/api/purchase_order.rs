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
    service::purchase_order::{PurchaseOrderService, CreatePurchaseOrderRequest, UpdatePurchaseOrderRequest, PurchaseOrderResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantOrgPath {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

pub fn create_purchase_order_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/purchase-orders", get(list_purchase_orders).post(create_purchase_order))
        .route("/purchase-orders/:id", get(get_purchase_order).put(update_purchase_order).delete(delete_purchase_order))
        .with_state(db)
}

async fn list_purchase_orders(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
) -> Result<ResponseJson<Response<Vec<PurchaseOrderResponse>>>, AppError> {
    let service = PurchaseOrderService::new(db);
    let orders = service.list_all(tenant_id, org_id).await?;
    Ok(ResponseJson(Response::success(orders)))
}

async fn create_purchase_order(
    State(db): State<DatabaseConnection>,
    Path(TenantOrgPath { tenant_id, org_id }): Path<TenantOrgPath>,
    Json(req): Json<CreatePurchaseOrderRequest>,
) -> Result<ResponseJson<Response<PurchaseOrderResponse>>, AppError> {
    let service = PurchaseOrderService::new(db);
    let order = service.create(tenant_id, org_id, req).await?;
    Ok(ResponseJson(Response::success(order)))
}

async fn get_purchase_order(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<PurchaseOrderResponse>>, AppError> {
    let service = PurchaseOrderService::new(db);
    let order = service.get_by_id(tenant_id, org_id, id).await?
        .ok_or_else(|| AppError::not_found("Purchase order not found".to_string()))?;
    Ok(ResponseJson(Response::success(order)))
}

async fn update_purchase_order(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
    Json(req): Json<UpdatePurchaseOrderRequest>,
) -> Result<ResponseJson<Response<PurchaseOrderResponse>>, AppError> {
    let service = PurchaseOrderService::new(db);
    let order = service.update(tenant_id, org_id, id, req).await?;
    Ok(ResponseJson(Response::success(order)))
}

async fn delete_purchase_order(
    State(db): State<DatabaseConnection>,
    Path((TenantOrgPath { tenant_id, org_id }, id)): Path<(TenantOrgPath, Uuid)>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = PurchaseOrderService::new(db);
    service.delete(tenant_id, org_id, id).await?;
    Ok(ResponseJson(Response::success(())))
}
