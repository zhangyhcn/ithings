use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

use crate::entity::{InventoryEntity, InventoryColumn, InventoryModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub warehouse_id: String,
    pub warehouse_name: Option<String>,
    pub location_id: Option<String>,
    pub location_code: Option<String>,
    pub material_id: String,
    pub material_code: Option<String>,
    pub material_name: Option<String>,
    pub batch_no: Option<String>,
    pub quantity: f64,
    pub frozen_qty: f64,
    pub available_qty: f64,
    pub cost_price: Option<f64>,
    pub production_date: Option<String>,
    pub expiry_date: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for InventoryResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            warehouse_id: model.warehouse_id.to_string(),
            warehouse_name: None, // 需要join查询填充
            location_id: model.location_id.map(|id| id.to_string()),
            location_code: None,
            material_id: model.material_id.to_string(),
            material_code: None,
            material_name: None,
            batch_no: model.batch_no,
            quantity: model.quantity.to_string().parse().unwrap(),
            frozen_qty: model.frozen_qty.to_string().parse().unwrap(),
            available_qty: model.available_qty.to_string().parse().unwrap(),
            cost_price: model.cost_price.map(|d| d.to_string().parse().unwrap()),
            production_date: model.production_date.map(|d| d.to_string()),
            expiry_date: model.expiry_date.map(|d| d.to_string()),
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct InventoryService {
    db: DatabaseConnection,
}

impl InventoryService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn list_all(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
    ) -> Result<Vec<InventoryResponse>, AppError> {
        let inventory = InventoryEntity::find()
            .filter(InventoryColumn::TenantId.eq(tenant_id))
            .filter(InventoryColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await?;

        Ok(inventory.into_iter().map(|i| i.into()).collect())
    }

    pub async fn list_by_warehouse(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        warehouse_id: Uuid,
    ) -> Result<Vec<InventoryResponse>, AppError> {
        let inventory = InventoryEntity::find()
            .filter(InventoryColumn::TenantId.eq(tenant_id))
            .filter(InventoryColumn::OrgId.eq(org_id))
            .filter(InventoryColumn::WarehouseId.eq(warehouse_id))
            .all(&self.db)
            .await?;

        Ok(inventory.into_iter().map(|i| i.into()).collect())
    }

    pub async fn list_by_material(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        material_id: Uuid,
    ) -> Result<Vec<InventoryResponse>, AppError> {
        let inventory = InventoryEntity::find()
            .filter(InventoryColumn::TenantId.eq(tenant_id))
            .filter(InventoryColumn::OrgId.eq(org_id))
            .filter(InventoryColumn::MaterialId.eq(material_id))
            .all(&self.db)
            .await?;

        Ok(inventory.into_iter().map(|i| i.into()).collect())
    }

    pub async fn get_by_id(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<InventoryResponse>, AppError> {
        let inventory = InventoryEntity::find()
            .filter(InventoryColumn::TenantId.eq(tenant_id))
            .filter(InventoryColumn::OrgId.eq(org_id))
            .filter(InventoryColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        Ok(inventory.map(|i| i.into()))
    }
}
