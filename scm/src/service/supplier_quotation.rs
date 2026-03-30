use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::entity::{SupplierQuotationEntity, SupplierQuotationColumn, SupplierQuotationModel as Model};
use crate::entity::supplier_quotation::ActiveModel;
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSupplierQuotationRequest {
    pub supplier_id: String,
    pub material_id: String,
    pub price: String,
    pub currency: Option<String>,
    pub min_qty: Option<String>,
    pub max_qty: Option<String>,
    pub valid_from: String,
    pub valid_to: String,
    pub lead_time: Option<i32>,
    pub payment_terms: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSupplierQuotationRequest {
    pub price: Option<String>,
    pub currency: Option<String>,
    pub min_qty: Option<String>,
    pub max_qty: Option<String>,
    pub valid_from: Option<String>,
    pub valid_to: Option<String>,
    pub lead_time: Option<i32>,
    pub payment_terms: Option<String>,
    pub remarks: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierQuotationResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub supplier_id: String,
    pub material_id: String,
    pub price: String,
    pub currency: String,
    pub min_qty: String,
    pub max_qty: Option<String>,
    pub valid_from: String,
    pub valid_to: String,
    pub lead_time: i32,
    pub payment_terms: Option<String>,
    pub remarks: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

pub struct SupplierQuotationService {
    db: DatabaseConnection,
}

impl SupplierQuotationService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_all(&self, tenant_id: Uuid, org_id: Uuid) -> Result<Vec<SupplierQuotationResponse>, AppError> {
        let items = SupplierQuotationEntity::find()
            .filter(SupplierQuotationColumn::TenantId.eq(tenant_id))
            .filter(SupplierQuotationColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(items.into_iter().map(Self::model_to_response).collect())
    }

    pub async fn create(&self, tenant_id: Uuid, org_id: Uuid, req: CreateSupplierQuotationRequest) -> Result<SupplierQuotationResponse, AppError> {
        let now = Utc::now().naive_utc();
        let id = Uuid::new_v4();
        
        let active_model = ActiveModel {
            id: Set(id),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            supplier_id: Set(Uuid::parse_str(&req.supplier_id).map_err(|_| AppError::BadRequest("Invalid supplier_id".to_string()))?),
            material_id: Set(Uuid::parse_str(&req.material_id).map_err(|_| AppError::BadRequest("Invalid material_id".to_string()))?),
            price: Set(req.price.parse().map_err(|_| AppError::BadRequest("Invalid price".to_string()))?),
            currency: Set(req.currency.unwrap_or_else(|| "CNY".to_string())),
            min_qty: Set(req.min_qty.map(|v| v.parse().unwrap_or(Decimal::ONE)).unwrap_or(Decimal::ONE)),
            max_qty: Set(req.max_qty.map(|v| v.parse().map_err(|_| AppError::BadRequest("Invalid max_qty".to_string()))).transpose()? ),
            valid_from: Set(req.valid_from.parse().map_err(|_| AppError::BadRequest("Invalid valid_from".to_string()))?),
            valid_to: Set(req.valid_to.parse().map_err(|_| AppError::BadRequest("Invalid valid_to".to_string()))?),
            lead_time: Set(req.lead_time.unwrap_or(0)),
            payment_terms: Set(req.payment_terms),
            remarks: Set(req.remarks),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn get(&self, id: Uuid) -> Result<SupplierQuotationResponse, AppError> {
        let model = SupplierQuotationEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?
            .ok_or_else(|| AppError::NotFound("Quotation not found".to_string()))?;

        Ok(Self::model_to_response(model))
    }

    pub async fn update(&self, id: Uuid, req: UpdateSupplierQuotationRequest) -> Result<SupplierQuotationResponse, AppError> {
        let model = SupplierQuotationEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?
            .ok_or_else(|| AppError::NotFound("Quotation not found".to_string()))?;

        let mut active_model: ActiveModel = model.into();
        
        if let Some(price) = req.price {
            active_model.price = Set(price.parse().map_err(|_| AppError::BadRequest("Invalid price".to_string()))?);
        }
        if let Some(currency) = req.currency {
            active_model.currency = Set(currency);
        }
        if let Some(min_qty) = req.min_qty {
            active_model.min_qty = Set(min_qty.parse().map_err(|_| AppError::BadRequest("Invalid min_qty".to_string()))?);
        }
        if let Some(max_qty) = req.max_qty {
            active_model.max_qty = Set(Some(max_qty.parse().map_err(|_| AppError::BadRequest("Invalid max_qty".to_string()))?));
        }
        if let Some(valid_from) = req.valid_from {
            active_model.valid_from = Set(valid_from.parse().map_err(|_| AppError::BadRequest("Invalid valid_from".to_string()))?);
        }
        if let Some(valid_to) = req.valid_to {
            active_model.valid_to = Set(valid_to.parse().map_err(|_| AppError::BadRequest("Invalid valid_to".to_string()))?);
        }
        if let Some(lead_time) = req.lead_time {
            active_model.lead_time = Set(lead_time);
        }
        if let Some(payment_terms) = req.payment_terms {
            active_model.payment_terms = Set(Some(payment_terms));
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
        SupplierQuotationEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| AppError::DatabaseError(e))?;

        Ok(())
    }

    fn model_to_response(model: Model) -> SupplierQuotationResponse {
        SupplierQuotationResponse {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            supplier_id: model.supplier_id.to_string(),
            material_id: model.material_id.to_string(),
            price: model.price.to_string(),
            currency: model.currency,
            min_qty: model.min_qty.to_string(),
            max_qty: model.max_qty.map(|v| v.to_string()),
            valid_from: model.valid_from.to_string(),
            valid_to: model.valid_to.to_string(),
            lead_time: model.lead_time,
            payment_terms: model.payment_terms,
            remarks: model.remarks,
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}
