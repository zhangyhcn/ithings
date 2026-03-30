use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set, QueryOrder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::entity::{StockMovementEntity, StockMovementColumn, StockMovementModel as Model};
use crate::utils::AppError;
use crate::service::inventory::InventoryService;
use rust_decimal::prelude::FromPrimitive;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStockMovementRequest {
    pub movement_type: String, // "in" | "out" | "transfer"
    pub work_order_id: Option<Uuid>,
    pub material_id: Uuid,
    pub quantity: f64,
    pub batch_no: Option<String>,
    pub warehouse_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub to_warehouse_id: Option<Uuid>,
    pub to_location_id: Option<Uuid>,
    pub operator_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockMovementResponse {
    pub id: String,
    pub tenant_id: String,
    pub movement_no: String,
    pub movement_type: String,
    pub work_order_id: Option<String>,
    pub material_id: String,
    pub quantity: String,
    pub batch_no: Option<String>,
    pub operator_id: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for StockMovementResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            movement_no: model.movement_no,
            movement_type: model.movement_type,
            work_order_id: model.work_order_id.map(|id| id.to_string()),
            material_id: model.material_id.to_string(),
            quantity: model.quantity.to_string(),
            batch_no: model.batch_no,
            operator_id: model.operator_id.map(|id| id.to_string()),
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct StockMovementService {
    db: Arc<DatabaseConnection>,
}

impl StockMovementService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    fn generate_movement_no() -> String {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S");
        format!("SM{}", timestamp)
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateStockMovementRequest,
    ) -> Result<StockMovementResponse, AppError> {
        let now = Utc::now().naive_utc();
        let movement_no = Self::generate_movement_no();

        // 创建出入库单
        let active_model = crate::entity::stock_movement::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            movement_no: Set(movement_no),
            movement_type: Set(req.movement_type.clone()),
            work_order_id: Set(req.work_order_id),
            material_id: Set(req.material_id),
            quantity: Set(Decimal::from_f64(req.quantity).unwrap_or(Decimal::ZERO)),
            batch_no: Set(req.batch_no.clone()),
            operator_id: Set(req.operator_id),
            status: Set("pending".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(self.db.as_ref()).await?;
        Ok(model.into())
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<StockMovementResponse>, AppError> {
        let models = StockMovementEntity::find()
            .filter(StockMovementColumn::TenantId.eq(tenant_id))
            .order_by_desc(StockMovementColumn::CreatedAt)
            .all(self.db.as_ref())
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<StockMovementResponse, AppError> {
        let model = StockMovementEntity::find()
            .filter(StockMovementColumn::Id.eq(id))
            .one(self.db.as_ref())
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::not_found("StockMovement not found".to_string())),
        }
    }

    pub async fn execute(&self, id: Uuid) -> Result<StockMovementResponse, AppError> {
        let model = StockMovementEntity::find()
            .filter(StockMovementColumn::Id.eq(id))
            .one(self.db.as_ref())
            .await?;

        let Some(model) = model else {
            return Err(AppError::not_found("StockMovement not found".to_string()));
        };

        if model.status != "pending" {
            return Err(AppError::bad_request("StockMovement already executed".to_string()));
        }

        // 更新库存
        let inventory_service = InventoryService::new(self.db.clone());
        
        match model.movement_type.as_str() {
            "in" => {
                inventory_service.adjust(model.tenant_id, crate::service::inventory::AdjustInventoryRequest {
                    material_id: model.material_id,
                    warehouse_id: None, // 从单据中获取，这里简化处理
                    location_id: None,
                    batch_no: model.batch_no.clone(),
                    quantity: model.quantity.to_string().parse().unwrap_or(0.0),
                    adjustment_type: "in".to_string(),
                }).await?;
            },
            "out" => {
                inventory_service.adjust(model.tenant_id, crate::service::inventory::AdjustInventoryRequest {
                    material_id: model.material_id,
                    warehouse_id: None,
                    location_id: None,
                    batch_no: model.batch_no.clone(),
                    quantity: model.quantity.to_string().parse().unwrap_or(0.0),
                    adjustment_type: "out".to_string(),
                }).await?;
            },
            _ => return Err(AppError::bad_request("Invalid movement type".to_string())),
        }

        // 更新单据状态
        let mut active_model = crate::entity::stock_movement::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            movement_no: Set(model.movement_no),
            movement_type: Set(model.movement_type),
            work_order_id: Set(model.work_order_id),
            material_id: Set(model.material_id),
            quantity: Set(model.quantity),
            batch_no: Set(model.batch_no),
            operator_id: Set(model.operator_id),
            status: Set("completed".to_string()),
            created_at: Set(model.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };

        let updated = active_model.update(self.db.as_ref()).await?;
        Ok(updated.into())
    }

    pub async fn cancel(&self, id: Uuid) -> Result<StockMovementResponse, AppError> {
        let model = StockMovementEntity::find()
            .filter(StockMovementColumn::Id.eq(id))
            .one(self.db.as_ref())
            .await?;

        let Some(model) = model else {
            return Err(AppError::not_found("StockMovement not found".to_string()));
        };

        if model.status != "pending" {
            return Err(AppError::bad_request("Only pending StockMovement can be cancelled".to_string()));
        }

        let mut active_model = crate::entity::stock_movement::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            movement_no: Set(model.movement_no),
            movement_type: Set(model.movement_type),
            work_order_id: Set(model.work_order_id),
            material_id: Set(model.material_id),
            quantity: Set(model.quantity),
            batch_no: Set(model.batch_no),
            operator_id: Set(model.operator_id),
            status: Set("cancelled".to_string()),
            created_at: Set(model.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };

        let updated = active_model.update(self.db.as_ref()).await?;
        Ok(updated.into())
    }
}
