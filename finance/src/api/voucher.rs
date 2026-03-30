use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    entity::{voucher, voucher_item},
    service::{VoucherService, CreateVoucherRequest, CreateVoucherItemRequest},
    AppState,
};
use sea_orm::prelude::Decimal;

#[derive(Debug, Deserialize)]
pub struct ListVouchersQuery {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct VoucherResponse {
    pub id: Uuid,
    pub voucher_no: String,
    pub voucher_date: chrono::NaiveDate,
    pub voucher_type: String,
    pub description: Option<String>,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<voucher::Model> for VoucherResponse {
    fn from(model: voucher::Model) -> Self {
        Self {
            id: model.id,
            voucher_no: model.voucher_no,
            voucher_date: model.voucher_date,
            voucher_type: model.voucher_type,
            description: model.description,
            total_debit: model.total_debit,
            total_credit: model.total_credit,
            status: model.status,
            created_at: model.created_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct VoucherDetailResponse {
    #[serde(flatten)]
    pub voucher: VoucherResponse,
    pub items: Vec<VoucherItemResponse>,
}

#[derive(Debug, Serialize)]
pub struct VoucherItemResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub description: Option<String>,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
}

impl From<voucher_item::Model> for VoucherItemResponse {
    fn from(model: voucher_item::Model) -> Self {
        Self {
            id: model.id,
            account_id: model.account_id,
            description: model.description,
            debit_amount: model.debit_amount,
            credit_amount: model.credit_amount,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateVoucherApiRequest {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
    pub voucher_date: chrono::NaiveDate,
    pub voucher_type: String,
    pub description: Option<String>,
    pub items: Vec<CreateVoucherItemApiRequest>,
}

#[derive(Debug, Deserialize)]
pub struct CreateVoucherItemApiRequest {
    pub account_id: Uuid,
    pub description: Option<String>,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
}

pub async fn list_vouchers(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListVouchersQuery>,
) -> Result<Json<Vec<VoucherResponse>>, StatusCode> {
    let service = VoucherService::new(state.db.clone());
    
    match service.list(query.tenant_id, query.org_id).await {
        Ok(vouchers) => Ok(Json(vouchers.into_iter().map(VoucherResponse::from).collect())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_voucher(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<VoucherDetailResponse>, StatusCode> {
    let service = VoucherService::new(state.db.clone());
    
    let voucher = service.get_by_id(id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    let items = service.get_items(id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(VoucherDetailResponse {
        voucher: VoucherResponse::from(voucher),
        items: items.into_iter().map(VoucherItemResponse::from).collect(),
    }))
}

pub async fn create_voucher(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateVoucherApiRequest>,
) -> Result<Json<VoucherResponse>, StatusCode> {
    let service = VoucherService::new(state.db.clone());
    
    let create_req = CreateVoucherRequest {
        tenant_id: req.tenant_id,
        org_id: req.org_id,
        voucher_date: req.voucher_date,
        voucher_type: req.voucher_type,
        description: req.description,
        items: req.items.into_iter().map(|item| CreateVoucherItemRequest {
            account_id: item.account_id,
            description: item.description,
            debit_amount: item.debit_amount,
            credit_amount: item.credit_amount,
        }).collect(),
    };
    
    match service.create(create_req).await {
        Ok(voucher) => Ok(Json(VoucherResponse::from(voucher))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn approve_voucher(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<VoucherResponse>, StatusCode> {
    let service = VoucherService::new(state.db.clone());
    let approved_by = Uuid::new_v4(); // TODO: 从认证信息中获取
    
    match service.approve(id, approved_by).await {
        Ok(voucher) => Ok(Json(VoucherResponse::from(voucher))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn delete_voucher(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let service = VoucherService::new(state.db.clone());
    
    match service.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
