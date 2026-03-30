use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::entity::{ContractEntity, ContractColumn, ContractModel as Model};
use crate::entity::contract::ActiveModel;
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContractRequest {
    pub contract_no: String,
    pub supplier_id: String,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub total_amount: String,
    pub currency: Option<String>,
    pub payment_terms: Option<String>,
    pub delivery_terms: Option<String>,
    pub quality_terms: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateContractRequest {
    pub title: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub total_amount: Option<String>,
    pub currency: Option<String>,
    pub payment_terms: Option<String>,
    pub delivery_terms: Option<String>,
    pub quality_terms: Option<String>,
    pub remarks: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub contract_no: String,
    pub supplier_id: String,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub total_amount: String,
    pub currency: String,
    pub payment_terms: Option<String>,
    pub delivery_terms: Option<String>,
    pub quality_terms: Option<String>,
    pub remarks: Option<String>,
    pub status: String,
    pub signed_by: Option<String>,
    pub signed_at: Option<String>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub struct ContractService {
    db: DatabaseConnection,
}

impl ContractService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_all(&self, tenant_id: Uuid, org_id: Uuid) -> Result<Vec<ContractResponse>, AppError> {
        let items = ContractEntity::find()
            .filter(ContractColumn::TenantId.eq(tenant_id))
            .filter(ContractColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(items.into_iter().map(Self::model_to_response).collect())
    }

    pub async fn create(&self, tenant_id: Uuid, org_id: Uuid, req: CreateContractRequest) -> Result<ContractResponse, AppError> {
        let now = Utc::now().naive_utc();
        let id = Uuid::new_v4();
        
        let active_model = ActiveModel {
            id: Set(id),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            contract_no: Set(req.contract_no),
            supplier_id: Set(Uuid::parse_str(&req.supplier_id).map_err(|_| AppError::BadRequest("Invalid supplier_id".to_string()))?),
            title: Set(req.title),
            start_date: Set(req.start_date.parse().map_err(|_| AppError::BadRequest("Invalid start_date".to_string()))?),
            end_date: Set(req.end_date.parse().map_err(|_| AppError::BadRequest("Invalid end_date".to_string()))?),
            total_amount: Set(req.total_amount.parse().map_err(|_| AppError::BadRequest("Invalid total_amount".to_string()))?),
            currency: Set(req.currency.unwrap_or_else(|| "CNY".to_string())),
            payment_terms: Set(req.payment_terms),
            delivery_terms: Set(req.delivery_terms),
            quality_terms: Set(req.quality_terms),
            remarks: Set(req.remarks),
            status: Set("draft".to_string()),
            signed_by: Set(None),
            signed_at: Set(None),
            created_by: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn get(&self, id: Uuid) -> Result<ContractResponse, AppError> {
        let model = ContractEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?
            .ok_or_else(|| AppError::NotFound("Contract not found".to_string()))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn update(&self, id: Uuid, req: UpdateContractRequest) -> Result<ContractResponse, AppError> {
        let model = ContractEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?
            .ok_or_else(|| AppError::NotFound("Contract not found".to_string()))?;

        let mut active_model: ActiveModel = model.into();
        
        if let Some(title) = req.title {
            active_model.title = Set(title);
        }
        if let Some(start_date) = req.start_date {
            active_model.start_date = Set(start_date.parse().map_err(|_| AppError::BadRequest("Invalid start_date".to_string()))?);
        }
        if let Some(end_date) = req.end_date {
            active_model.end_date = Set(end_date.parse().map_err(|_| AppError::BadRequest("Invalid end_date".to_string()))?);
        }
        if let Some(total_amount) = req.total_amount {
            active_model.total_amount = Set(total_amount.parse().map_err(|_| AppError::BadRequest("Invalid total_amount".to_string()))?);
        }
        if let Some(currency) = req.currency {
            active_model.currency = Set(currency);
        }
        if let Some(payment_terms) = req.payment_terms {
            active_model.payment_terms = Set(Some(payment_terms));
        }
        if let Some(delivery_terms) = req.delivery_terms {
            active_model.delivery_terms = Set(Some(delivery_terms));
        }
        if let Some(quality_terms) = req.quality_terms {
            active_model.quality_terms = Set(Some(quality_terms));
        }
        if let Some(remarks) = req.remarks {
            active_model.remarks = Set(Some(remarks));
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }

        active_model.updated_at = Set(Utc::now().naive_utc());

        let model = active_model
            .update(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        ContractEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(())
    }

    fn model_to_response(model: Model) -> ContractResponse {
        ContractResponse {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            contract_no: model.contract_no,
            supplier_id: model.supplier_id.to_string(),
            title: model.title,
            start_date: model.start_date.to_string(),
            end_date: model.end_date.to_string(),
            total_amount: model.total_amount.to_string(),
            currency: model.currency,
            payment_terms: model.payment_terms,
            delivery_terms: model.delivery_terms,
            quality_terms: model.quality_terms,
            remarks: model.remarks,
            status: model.status,
            signed_by: model.signed_by.map(|v| v.to_string()),
            signed_at: model.signed_at.map(|v| v.to_string()),
            created_by: model.created_by.map(|v| v.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
