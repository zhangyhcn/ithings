use crate::entity::{production_record, work_order};
use crate::service::work_order::WorkOrderService;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sea_orm::*;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use uuid::Uuid;

pub struct ProductionRecordService {
    db: Arc<DatabaseConnection>,
}

impl ProductionRecordService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        work_order_id: Uuid,
        process_id: Uuid,
        equipment_id: Option<Uuid>,
        operator_id: Option<Uuid>,
        batch_no: Option<String>,
        quantity: Decimal,
        start_time: Option<chrono::NaiveDateTime>,
        process_data: Option<JsonValue>,
    ) -> Result<production_record::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let record = production_record::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            work_order_id: Set(work_order_id),
            process_id: Set(process_id),
            equipment_id: Set(equipment_id),
            operator_id: Set(operator_id),
            batch_no: Set(batch_no),
            quantity: Set(quantity),
            good_qty: Set(None),
            defect_qty: Set(None),
            start_time: Set(start_time),
            end_time: Set(None),
            process_data: Set(process_data),
            created_at: Set(now),
            updated_at: Set(now),
        };

        record.insert(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        good_qty: Option<Decimal>,
        defect_qty: Option<Decimal>,
        end_time: Option<chrono::NaiveDateTime>,
        process_data: Option<JsonValue>,
    ) -> Result<production_record::Model, DbErr> {
        let record = production_record::Entity::find_by_id(id)
            .filter(production_record::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Production record not found".into()))?;

        let mut record: production_record::ActiveModel = record.into();
        record.good_qty = Set(good_qty);
        record.defect_qty = Set(defect_qty);
        record.end_time = Set(end_time);
        record.process_data = Set(process_data);
        record.updated_at = Set(chrono::Utc::now().naive_utc());

        record.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<production_record::Model>, DbErr> {
        production_record::Entity::find_by_id(id)
            .filter(production_record::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_by_work_order(
        &self,
        tenant_id: Uuid,
        work_order_id: Uuid,
    ) -> Result<Vec<production_record::Model>, DbErr> {
        production_record::Entity::find()
            .filter(production_record::Column::TenantId.eq(tenant_id))
            .filter(production_record::Column::WorkOrderId.eq(work_order_id))
            .order_by_desc(production_record::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_by_process(
        &self,
        tenant_id: Uuid,
        process_id: Uuid,
    ) -> Result<Vec<production_record::Model>, DbErr> {
        production_record::Entity::find()
            .filter(production_record::Column::TenantId.eq(tenant_id))
            .filter(production_record::Column::ProcessId.eq(process_id))
            .order_by_desc(production_record::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<production_record::Model>, DbErr> {
        production_record::Entity::find()
            .filter(production_record::Column::TenantId.eq(tenant_id))
            .order_by_desc(production_record::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        production_record::Entity::delete_by_id(id)
            .filter(production_record::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
