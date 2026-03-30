use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entity::{SupplierEntity, SupplierColumn, SupplierModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSupplierRequest {
    pub supplier_code: String,
    pub supplier_name: String,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub address: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account: Option<String>,
    pub tax_number: Option<String>,
    pub supplier_type: Option<String>,
    pub credit_level: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSupplierRequest {
    pub supplier_name: Option<String>,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub address: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account: Option<String>,
    pub tax_number: Option<String>,
    pub supplier_type: Option<String>,
    pub credit_level: Option<String>,
    pub remarks: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SupplierResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub supplier_code: String,
    pub supplier_name: String,
    pub contact_person: Option<String>,
    pub contact_phone: Option<String>,
    pub contact_email: Option<String>,
    pub address: Option<String>,
    pub bank_name: Option<String>,
    pub bank_account: Option<String>,
    pub tax_number: Option<String>,
    pub supplier_type: Option<String>,
    pub credit_level: Option<String>,
    pub remarks: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for SupplierResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            supplier_code: model.supplier_code,
            supplier_name: model.supplier_name,
            contact_person: model.contact_person,
            contact_phone: model.contact_phone,
            contact_email: model.contact_email,
            address: model.address,
            bank_name: model.bank_name,
            bank_account: model.bank_account,
            tax_number: model.tax_number,
            supplier_type: model.supplier_type,
            credit_level: model.credit_level,
            remarks: model.remarks,
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct SupplierService {
    db: DatabaseConnection,
}

impl SupplierService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        req: CreateSupplierRequest,
    ) -> Result<SupplierResponse, AppError> {
        let now = Utc::now().naive_utc();
        let active_model = crate::entity::supplier::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            supplier_code: Set(req.supplier_code),
            supplier_name: Set(req.supplier_name),
            contact_person: Set(req.contact_person),
            contact_phone: Set(req.contact_phone),
            contact_email: Set(req.contact_email),
            address: Set(req.address),
            bank_name: Set(req.bank_name),
            bank_account: Set(req.bank_account),
            tax_number: Set(req.tax_number),
            supplier_type: Set(req.supplier_type),
            credit_level: Set(req.credit_level),
            remarks: Set(req.remarks),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn list_all(&self, tenant_id: Uuid, org_id: Uuid) -> Result<Vec<SupplierResponse>, AppError> {
        let models = SupplierEntity::find()
            .filter(SupplierColumn::TenantId.eq(tenant_id))
            .filter(SupplierColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, org_id: Uuid, id: Uuid) -> Result<Option<SupplierResponse>, AppError> {
        let model = SupplierEntity::find_by_id(id)
            .filter(SupplierColumn::TenantId.eq(tenant_id))
            .filter(SupplierColumn::OrgId.eq(org_id))
            .one(&self.db)
            .await?;

        Ok(model.map(Into::into))
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
        req: UpdateSupplierRequest,
    ) -> Result<SupplierResponse, AppError> {
        let model = SupplierEntity::find_by_id(id)
            .filter(SupplierColumn::TenantId.eq(tenant_id))
            .filter(SupplierColumn::OrgId.eq(org_id))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::not_found("Supplier not found".to_string()))?;

        let mut active_model: crate::entity::supplier::ActiveModel = model.into();
        
        if let Some(name) = req.supplier_name {
            active_model.supplier_name = Set(name);
        }
        if let Some(person) = req.contact_person {
            active_model.contact_person = Set(Some(person));
        }
        if let Some(phone) = req.contact_phone {
            active_model.contact_phone = Set(Some(phone));
        }
        if let Some(email) = req.contact_email {
            active_model.contact_email = Set(Some(email));
        }
        if let Some(addr) = req.address {
            active_model.address = Set(Some(addr));
        }
        if let Some(bank) = req.bank_name {
            active_model.bank_name = Set(Some(bank));
        }
        if let Some(account) = req.bank_account {
            active_model.bank_account = Set(Some(account));
        }
        if let Some(tax) = req.tax_number {
            active_model.tax_number = Set(Some(tax));
        }
        if let Some(sup_type) = req.supplier_type {
            active_model.supplier_type = Set(Some(sup_type));
        }
        if let Some(level) = req.credit_level {
            active_model.credit_level = Set(Some(level));
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
        SupplierEntity::delete_by_id(id)
            .filter(SupplierColumn::TenantId.eq(tenant_id))
            .filter(SupplierColumn::OrgId.eq(org_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
