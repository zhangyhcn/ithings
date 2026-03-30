use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entity::{InboundOrderEntity, InboundOrderColumn, InboundOrderModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInboundOrderRequest {
    pub order_type: String,
    pub warehouse_id: String,
    pub supplier_id: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InboundOrderResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub order_no: String,
    pub order_type: String,
    pub warehouse_id: String,
    pub supplier_id: Option<String>,
    pub status: String,
    pub total_qty: String,
    pub remark: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for InboundOrderResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            order_no: model.order_no,
            order_type: model.order_type,
            warehouse_id: model.warehouse_id.to_string(),
            supplier_id: model.supplier_id.map(|id| id.to_string()),
            status: model.status,
            total_qty: model.total_qty.to_string(),
            remark: model.remark,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct InboundOrderService {
    db: DatabaseConnection,
}

impl InboundOrderService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        req: CreateInboundOrderRequest,
    ) -> Result<InboundOrderResponse, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now().naive_utc();
        let order_no = format!("IN{}", now.format("%Y%m%d%H%M%S"));

        let inbound = crate::entity::inbound_order::ActiveModel {
            id: Set(id),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            order_no: Set(order_no),
            order_type: Set(req.order_type),
            source_order_id: Set(None),
            warehouse_id: Set(Uuid::parse_str(&req.warehouse_id).unwrap()),
            supplier_id: Set(req.supplier_id.and_then(|s| Uuid::parse_str(&s).ok())),
            status: Set("draft".to_string()),
            total_qty: Set(rust_decimal::Decimal::ZERO),
            remark: Set(req.remark),
            created_by: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let inbound = inbound.insert(&self.db).await?;
        Ok(inbound.into())
    }

    pub async fn list_all(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
    ) -> Result<Vec<InboundOrderResponse>, AppError> {
        let orders = InboundOrderEntity::find()
            .filter(InboundOrderColumn::TenantId.eq(tenant_id))
            .filter(InboundOrderColumn::OrgId.eq(org_id))
            .all(&self.db)
            .await?;

        Ok(orders.into_iter().map(|o| o.into()).collect())
    }

    pub async fn delete(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<(), AppError> {
        InboundOrderEntity::delete_by_id(id)
            .filter(InboundOrderColumn::TenantId.eq(tenant_id))
            .filter(InboundOrderColumn::OrgId.eq(org_id))
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
