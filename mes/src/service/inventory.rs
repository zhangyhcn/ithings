use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set, QueryOrder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::entity::{InventoryEntity, InventoryColumn, InventoryModel as Model};
use crate::utils::AppError;
use rust_decimal::prelude::FromPrimitive;

#[derive(Debug, Serialize, Deserialize)]
pub struct AdjustInventoryRequest {
    pub material_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub batch_no: Option<String>,
    pub quantity: f64,
    pub adjustment_type: String, // "in" | "out" | "adjust"
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockInventoryRequest {
    pub material_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub batch_no: Option<String>,
    pub quantity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryResponse {
    pub id: String,
    pub tenant_id: String,
    pub material_id: String,
    pub warehouse_id: Option<String>,
    pub location_id: Option<String>,
    pub batch_no: Option<String>,
    pub quantity: String,
    pub locked_qty: String,
    pub available_qty: String,
    pub updated_at: String,
}

impl From<Model> for InventoryResponse {
    fn from(model: Model) -> Self {
        let available_qty = model.quantity - model.locked_qty;
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            material_id: model.material_id.to_string(),
            warehouse_id: model.warehouse_id.map(|id| id.to_string()),
            location_id: model.location_id.map(|id| id.to_string()),
            batch_no: model.batch_no,
            quantity: model.quantity.to_string(),
            locked_qty: model.locked_qty.to_string(),
            available_qty: available_qty.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct InventoryService {
    db: Arc<DatabaseConnection>,
}

impl InventoryService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<InventoryResponse>, AppError> {
        let models = InventoryEntity::find()
            .filter(InventoryColumn::TenantId.eq(tenant_id))
            .order_by_desc(InventoryColumn::UpdatedAt)
            .all(self.db.as_ref())
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn list_by_material(&self, tenant_id: Uuid, material_id: Uuid) -> Result<Vec<InventoryResponse>, AppError> {
        let models = InventoryEntity::find()
            .filter(InventoryColumn::TenantId.eq(tenant_id))
            .filter(InventoryColumn::MaterialId.eq(material_id))
            .order_by_desc(InventoryColumn::UpdatedAt)
            .all(self.db.as_ref())
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<InventoryResponse, AppError> {
        let model = InventoryEntity::find()
            .filter(InventoryColumn::Id.eq(id))
            .one(self.db.as_ref())
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::not_found("Inventory not found".to_string())),
        }
    }

    pub async fn find_or_create(
        &self,
        tenant_id: Uuid,
        material_id: Uuid,
        warehouse_id: Option<Uuid>,
        location_id: Option<Uuid>,
        batch_no: Option<String>,
    ) -> Result<Model, AppError> {
        // 尝试查找现有库存
        let mut query = InventoryEntity::find()
            .filter(InventoryColumn::TenantId.eq(tenant_id))
            .filter(InventoryColumn::MaterialId.eq(material_id));

        if let Some(wh_id) = warehouse_id {
            query = query.filter(InventoryColumn::WarehouseId.eq(wh_id));
        }
        if let Some(loc_id) = location_id {
            query = query.filter(InventoryColumn::LocationId.eq(loc_id));
        }
        if let Some(ref batch) = batch_no {
            query = query.filter(InventoryColumn::BatchNo.eq(batch));
        }

        let model = query.one(self.db.as_ref()).await?;

        match model {
            Some(model) => Ok(model),
            None => {
                // 创建新库存记录
                let now = Utc::now().naive_utc();
                let active_model = crate::entity::inventory::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    tenant_id: Set(tenant_id),
                    material_id: Set(material_id),
                    warehouse_id: Set(warehouse_id),
                    location_id: Set(location_id),
                    batch_no: Set(batch_no),
                    quantity: Set(Decimal::ZERO),
                    locked_qty: Set(Decimal::ZERO),
                    updated_at: Set(now),
                };
                Ok(active_model.insert(self.db.as_ref()).await?)
            }
        }
    }

    pub async fn adjust(
        &self,
        tenant_id: Uuid,
        req: AdjustInventoryRequest,
    ) -> Result<InventoryResponse, AppError> {
        let inventory = self.find_or_create(
            tenant_id,
            req.material_id,
            req.warehouse_id,
            req.location_id,
            req.batch_no,
        ).await?;

        let quantity_decimal = Decimal::from_f64(req.quantity).unwrap_or(Decimal::ZERO);
        let new_quantity = match req.adjustment_type.as_str() {
            "in" => inventory.quantity + quantity_decimal,
            "out" => {
                if inventory.quantity < quantity_decimal {
                    return Err(AppError::bad_request("Insufficient inventory".to_string()));
                }
                inventory.quantity - quantity_decimal
            },
            "adjust" => quantity_decimal,
            _ => return Err(AppError::bad_request("Invalid adjustment type".to_string())),
        };

        let mut active_model = crate::entity::inventory::ActiveModel {
            id: Set(inventory.id),
            tenant_id: Set(inventory.tenant_id),
            material_id: Set(inventory.material_id),
            warehouse_id: Set(inventory.warehouse_id),
            location_id: Set(inventory.location_id),
            batch_no: Set(inventory.batch_no),
            quantity: Set(new_quantity),
            locked_qty: Set(inventory.locked_qty),
            updated_at: Set(Utc::now().naive_utc()),
        };

        let updated = active_model.update(self.db.as_ref()).await?;
        Ok(updated.into())
    }

    pub async fn lock(
        &self,
        tenant_id: Uuid,
        req: LockInventoryRequest,
    ) -> Result<InventoryResponse, AppError> {
        let inventory = self.find_or_create(
            tenant_id,
            req.material_id,
            req.warehouse_id,
            req.location_id,
            req.batch_no,
        ).await?;

        let lock_qty = Decimal::from_f64(req.quantity).unwrap_or(Decimal::ZERO);
        let available_qty = inventory.quantity - inventory.locked_qty;

        if available_qty < lock_qty {
            return Err(AppError::bad_request("Insufficient available inventory".to_string()));
        }

        let mut active_model = crate::entity::inventory::ActiveModel {
            id: Set(inventory.id),
            tenant_id: Set(inventory.tenant_id),
            material_id: Set(inventory.material_id),
            warehouse_id: Set(inventory.warehouse_id),
            location_id: Set(inventory.location_id),
            batch_no: Set(inventory.batch_no),
            quantity: Set(inventory.quantity),
            locked_qty: Set(inventory.locked_qty + lock_qty),
            updated_at: Set(Utc::now().naive_utc()),
        };

        let updated = active_model.update(self.db.as_ref()).await?;
        Ok(updated.into())
    }

    pub async fn unlock(
        &self,
        tenant_id: Uuid,
        material_id: Uuid,
        warehouse_id: Option<Uuid>,
        location_id: Option<Uuid>,
        batch_no: Option<String>,
        quantity: f64,
    ) -> Result<InventoryResponse, AppError> {
        let inventory = self.find_or_create(
            tenant_id,
            material_id,
            warehouse_id,
            location_id,
            batch_no,
        ).await?;

        let unlock_qty = Decimal::from_f64(quantity).unwrap_or(Decimal::ZERO);

        if inventory.locked_qty < unlock_qty {
            return Err(AppError::bad_request("Cannot unlock more than locked quantity".to_string()));
        }

        let mut active_model = crate::entity::inventory::ActiveModel {
            id: Set(inventory.id),
            tenant_id: Set(inventory.tenant_id),
            material_id: Set(inventory.material_id),
            warehouse_id: Set(inventory.warehouse_id),
            location_id: Set(inventory.location_id),
            batch_no: Set(inventory.batch_no),
            quantity: Set(inventory.quantity),
            locked_qty: Set(inventory.locked_qty - unlock_qty),
            updated_at: Set(Utc::now().naive_utc()),
        };

        let updated = active_model.update(self.db.as_ref()).await?;
        Ok(updated.into())
    }
}
