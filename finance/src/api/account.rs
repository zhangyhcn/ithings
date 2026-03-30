use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{entity::account, service::AccountService, AppState};

#[derive(Debug, Deserialize)]
pub struct ListAccountsQuery {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub level: i32,
    pub is_leaf: bool,
    pub debit_credit: String,
    pub currency: String,
    pub status: String,
    pub remarks: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<account::Model> for AccountResponse {
    fn from(model: account::Model) -> Self {
        Self {
            id: model.id,
            account_code: model.account_code,
            account_name: model.account_name,
            account_type: model.account_type,
            parent_id: model.parent_id,
            level: model.level,
            is_leaf: model.is_leaf,
            debit_credit: model.debit_credit,
            currency: model.currency,
            status: model.status,
            remarks: model.remarks,
            created_at: model.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub tenant_id: Uuid,
    pub org_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub level: i32,
    pub is_leaf: bool,
    pub debit_credit: String,
    pub currency: Option<String>,
    pub remarks: Option<String>,
}

pub async fn list_accounts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListAccountsQuery>,
) -> Result<Json<Vec<AccountResponse>>, StatusCode> {
    let service = AccountService::new(state.db.clone());
    
    match service.list(query.tenant_id, query.org_id).await {
        Ok(accounts) => Ok(Json(accounts.into_iter().map(AccountResponse::from).collect())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_account(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<AccountResponse>, StatusCode> {
    let service = AccountService::new(state.db.clone());
    
    match service.get_by_id(id).await {
        Ok(account) => Ok(Json(AccountResponse::from(account))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateAccountRequest>,
) -> Result<Json<AccountResponse>, StatusCode> {
    let service = AccountService::new(state.db.clone());
    
    let account_model = account::ActiveModel {
        id: sea_orm::ActiveValue::NotSet,
        tenant_id: sea_orm::ActiveValue::Set(req.tenant_id),
        org_id: sea_orm::ActiveValue::Set(req.org_id),
        account_code: sea_orm::ActiveValue::Set(req.account_code),
        account_name: sea_orm::ActiveValue::Set(req.account_name),
        account_type: sea_orm::ActiveValue::Set(req.account_type),
        parent_id: sea_orm::ActiveValue::Set(req.parent_id),
        level: sea_orm::ActiveValue::Set(req.level),
        is_leaf: sea_orm::ActiveValue::Set(req.is_leaf),
        debit_credit: sea_orm::ActiveValue::Set(req.debit_credit),
        currency: sea_orm::ActiveValue::Set(req.currency.unwrap_or_else(|| "CNY".to_string())),
        status: sea_orm::ActiveValue::Set("active".to_string()),
        remarks: sea_orm::ActiveValue::Set(req.remarks),
        created_at: sea_orm::ActiveValue::NotSet,
        updated_at: sea_orm::ActiveValue::NotSet,
    };
    
    match service.create(account_model).await {
        Ok(account) => Ok(Json(AccountResponse::from(account))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

pub async fn delete_account(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let service = AccountService::new(state.db.clone());
    
    match service.delete(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
