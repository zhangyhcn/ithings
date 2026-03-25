use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

use crate::entity::{WorkOrderEntity, WorkOrderColumn, WorkOrderModel as Model};
use crate::utils::AppError;
use rust_decimal::prelude::FromPrimitive;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWorkOrderRequest {
    pub erp_order_no: Option<String>,
    pub product_id: Uuid,
    pub product_name: String,
    pub quantity: f64,
    pub priority: Option<i32>,
    pub plan_start_time: Option<String>,
    pub plan_end_time: Option<String>,
    pub workshop_id: Option<Uuid>,
    pub production_line_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWorkOrderRequest {
    pub quantity: Option<f64>,
    pub priority: Option<i32>,
    pub plan_start_time: Option<String>,
    pub plan_end_time: Option<String>,
    pub workshop_id: Option<Uuid>,
    pub production_line_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkOrderResponse {
    pub id: String,
    pub tenant_id: String,
    pub order_no: String,
    pub erp_order_no: Option<String>,
    pub product_id: String,
    pub product_name: String,
    pub quantity: String,
    pub completed_qty: String,
    pub status: String,
    pub priority: i32,
    pub plan_start_time: Option<String>,
    pub plan_end_time: Option<String>,
    pub actual_start_time: Option<String>,
    pub actual_end_time: Option<String>,
    pub workshop_id: Option<String>,
    pub production_line_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for WorkOrderResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            order_no: model.order_no,
            erp_order_no: model.erp_order_no,
            product_id: model.product_id.to_string(),
            product_name: model.product_name,
            quantity: model.quantity.to_string(),
            completed_qty: model.completed_qty.to_string(),
            status: model.status,
            priority: model.priority,
            plan_start_time: model.plan_start_time.map(|t| t.to_string()),
            plan_end_time: model.plan_end_time.map(|t| t.to_string()),
            actual_start_time: model.actual_start_time.map(|t| t.to_string()),
            actual_end_time: model.actual_end_time.map(|t| t.to_string()),
            workshop_id: model.workshop_id.map(|id| id.to_string()),
            production_line_id: model.production_line_id.map(|id| id.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct WorkOrderService {
    db: DatabaseConnection,
}

impl WorkOrderService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn generate_order_no() -> String {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S");
        format!("WO{}", timestamp)
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateWorkOrderRequest,
    ) -> Result<WorkOrderResponse, AppError> {
        let now = Utc::now().naive_utc();
        let order_no = Self::generate_order_no();

        let active_model = crate::entity::work_order::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            order_no: Set(order_no),
            erp_order_no: Set(req.erp_order_no),
            product_id: Set(req.product_id),
            product_name: Set(req.product_name),
            quantity: Set(Decimal::from_f64(req.quantity).unwrap_or(Decimal::ZERO)),
            completed_qty: Set(Decimal::ZERO),
            status: Set("pending".to_string()),
            priority: Set(req.priority.unwrap_or(0)),
            plan_start_time: Set(req.plan_start_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|dt| dt.naive_utc()))),
            plan_end_time: Set(req.plan_end_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|dt| dt.naive_utc()))),
            actual_start_time: Set(None),
            actual_end_time: Set(None),
            workshop_id: Set(req.workshop_id),
            production_line_id: Set(req.production_line_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<WorkOrderResponse>, AppError> {
        let models = WorkOrderEntity::find()
            .filter(WorkOrderColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<WorkOrderResponse, AppError> {
        let model = WorkOrderEntity::find()
            .filter(WorkOrderColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::not_found("WorkOrder not found".to_string())),
        }
    }

    pub async fn start(&self, id: Uuid) -> Result<WorkOrderResponse, AppError> {
        let model = WorkOrderEntity::find()
            .filter(WorkOrderColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(model) = model else {
            return Err(AppError::not_found("WorkOrder not found".to_string()));
        };

        let mut active_model = crate::entity::work_order::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            order_no: Set(model.order_no),
            erp_order_no: Set(model.erp_order_no),
            product_id: Set(model.product_id),
            product_name: Set(model.product_name),
            quantity: Set(model.quantity),
            completed_qty: Set(model.completed_qty),
            status: Set("in_progress".to_string()),
            priority: Set(model.priority),
            plan_start_time: Set(model.plan_start_time),
            plan_end_time: Set(model.plan_end_time),
            actual_start_time: Set(Some(Utc::now().naive_utc())),
            actual_end_time: Set(model.actual_end_time),
            workshop_id: Set(model.workshop_id),
            production_line_id: Set(model.production_line_id),
            created_at: Set(model.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };

        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }

    pub async fn complete(
        &self,
        id: Uuid,
        completed_qty: f64,
    ) -> Result<WorkOrderResponse, AppError> {
        let model = WorkOrderEntity::find()
            .filter(WorkOrderColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(model) = model else {
            return Err(AppError::not_found("WorkOrder not found".to_string()));
        };

        let status = if Decimal::from_f64(completed_qty).unwrap_or(Decimal::ZERO) >= model.quantity {
            "completed"
        } else {
            "in_progress"
        };

        let mut active_model = crate::entity::work_order::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            order_no: Set(model.order_no),
            erp_order_no: Set(model.erp_order_no),
            product_id: Set(model.product_id),
            product_name: Set(model.product_name),
            quantity: Set(model.quantity),
            completed_qty: Set(Decimal::from_f64(completed_qty).unwrap_or(Decimal::ZERO)),
            status: Set(status.to_string()),
            priority: Set(model.priority),
            plan_start_time: Set(model.plan_start_time),
            plan_end_time: Set(model.plan_end_time),
            actual_start_time: Set(model.actual_start_time),
            actual_end_time: Set(Some(Utc::now().naive_utc())),
            workshop_id: Set(model.workshop_id),
            production_line_id: Set(model.production_line_id),
            created_at: Set(model.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };

        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }
}
