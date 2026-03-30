use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use crate::entity::{PurchaseOrderEntity, PurchaseOrderColumn, PurchaseOrderModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePurchaseOrderRequest {
    pub supplier_id: String,
    pub order_date: String,
    pub expected_delivery_date: Option<String>,
    pub payment_terms: Option<String>,
    pub delivery_address: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub total_amount: f64,
    pub currency: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePurchaseOrderRequest {
    pub expected_delivery_date: Option<String>,
    pub payment_terms: Option<String>,
    pub delivery_address: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub total_amount: Option<f64>,
    pub currency: Option<String>,
    pub remarks: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderResponse {
    pub id: String,
    pub tenant_id: String,
    pub order_no: String,
    pub supplier_id: String,
    pub order_date: String,
    pub expected_delivery_date: Option<String>,
    pub payment_terms: Option<String>,
    pub delivery_address: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub total_amount: String,
    pub currency: Option<String>,
    pub remarks: Option<String>,
    pub status: String,
    pub created_by: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for PurchaseOrderResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            order_no: model.order_no,
            supplier_id: model.supplier_id.to_string(),
            order_date: model.order_date.to_string(),
            expected_delivery_date: model.expected_delivery_date.map(|d| d.to_string()),
            payment_terms: model.payment_terms,
            delivery_address: model.delivery_address,
            contact_person: model.contact_person,
            contact_phone: model.contact_phone,
            total_amount: model.total_amount.to_string(),
            currency: model.currency,
            remarks: model.remarks,
            status: model.status,
            created_by: model.created_by.map(|id| id.to_string()),
            approved_by: model.approved_by.map(|id| id.to_string()),
            approved_at: model.approved_at.map(|t| t.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct PurchaseOrderService {
    db: DatabaseConnection,
}

impl PurchaseOrderService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn generate_order_no() -> String {
        let now = Utc::now();
        let uuid_part = &Uuid::new_v4().to_string()[..8];
        format!("PO{}{}", now.format("%Y%m%d%H%M%S"), uuid_part)
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        req: CreatePurchaseOrderRequest,
    ) -> Result<PurchaseOrderResponse, AppError> {
        let now = Utc::now().naive_utc();
        let order_no = Self::generate_order_no();
        let supplier_id = Uuid::parse_str(&req.supplier_id)
            .map_err(|_| AppError::bad_request("Invalid supplier_id".to_string()))?;
        let order_date = chrono::NaiveDate::parse_from_str(&req.order_date, "%Y-%m-%d")
            .map_err(|_| AppError::bad_request("Invalid order_date format".to_string()))?;

        let active_model = crate::entity::purchase_order::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            order_no: Set(order_no),
            supplier_id: Set(supplier_id),
            order_date: Set(order_date),
            expected_delivery_date: Set(req.expected_delivery_date.and_then(|d| 
                chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()
            )),
            payment_terms: Set(req.payment_terms),
            delivery_address: Set(req.delivery_address),
            contact_person: Set(req.contact_person),
            contact_phone: Set(req.contact_phone),
            total_amount: Set(Decimal::from_f64(req.total_amount).unwrap_or(Decimal::ZERO)),
            currency: Set(req.currency),
            remarks: Set(req.remarks),
            status: Set("draft".to_string()),
            created_by: Set(None),
            approved_by: Set(None),
            approved_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn list_all(&self, tenant_id: Uuid, org_id: Uuid) -> Result<Vec<PurchaseOrderResponse>, AppError> {
        let models = PurchaseOrderEntity::find()
            .filter(PurchaseOrderColumn::TenantId.eq(tenant_id))
            .filter(PurchaseOrderColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, org_id: Uuid, id: Uuid) -> Result<Option<PurchaseOrderResponse>, AppError> {
        let model = PurchaseOrderEntity::find_by_id(id)
            .filter(PurchaseOrderColumn::TenantId.eq(tenant_id))
            .filter(PurchaseOrderColumn::OrgId.eq(org_id))
            .one(&self.db)
            .await?;

        Ok(model.map(Into::into))
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
        req: UpdatePurchaseOrderRequest,
    ) -> Result<PurchaseOrderResponse, AppError> {
        let model = PurchaseOrderEntity::find_by_id(id)
            .filter(PurchaseOrderColumn::TenantId.eq(tenant_id))
            .filter(PurchaseOrderColumn::OrgId.eq(org_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::not_found("Purchase order not found".to_string()))?;

        let mut active_model: crate::entity::purchase_order::ActiveModel = model.into();
        
        if let Some(date) = req.expected_delivery_date {
            active_model.expected_delivery_date = Set(
                chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok()
            );
        }
        if let Some(terms) = req.payment_terms {
            active_model.payment_terms = Set(Some(terms));
        }
        if let Some(addr) = req.delivery_address {
            active_model.delivery_address = Set(Some(addr));
        }
        if let Some(person) = req.contact_person {
            active_model.contact_person = Set(Some(person));
        }
        if let Some(phone) = req.contact_phone {
            active_model.contact_phone = Set(Some(phone));
        }
        if let Some(amount) = req.total_amount {
            active_model.total_amount = Set(Decimal::from_f64(amount).unwrap_or(Decimal::ZERO));
        }
        if let Some(currency) = req.currency {
            active_model.currency = Set(Some(currency));
        }
        if let Some(remark) = req.remarks {
            active_model.remarks = Set(Some(remark));
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }
        active_model.updated_at = Set(Utc::now().naive_utc());

        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }

    pub async fn delete(&self, tenant_id: Uuid, org_id: Uuid, id: Uuid) -> Result<(), AppError> {
        PurchaseOrderEntity::delete_by_id(id)
            .filter(PurchaseOrderColumn::TenantId.eq(tenant_id))
            .filter(PurchaseOrderColumn::OrgId.eq(org_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
