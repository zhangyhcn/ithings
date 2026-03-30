use crate::entity::schedule_plan;
use rust_decimal::Decimal;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct SchedulePlanService {
    db: Arc<DatabaseConnection>,
}

impl SchedulePlanService {
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
        plan_quantity: Decimal,
        start_time: Option<chrono::NaiveDateTime>,
        end_time: Option<chrono::NaiveDateTime>,
    ) -> Result<schedule_plan::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        
        // 生成排程编号
        let plan_no = format!("SP{}", now.format("%Y%m%d%H%M%S"));
        
        let plan = schedule_plan::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            plan_no: Set(plan_no),
            work_order_id: Set(work_order_id),
            process_id: Set(process_id),
            equipment_id: Set(equipment_id),
            operator_id: Set(operator_id),
            plan_quantity: Set(plan_quantity),
    
            status: Set("pending".to_string()),
            start_time: Set(start_time),
            end_time: Set(end_time),
            created_at: Set(now),
            updated_at: Set(now),
        };

        plan.insert(self.db.as_ref()).await
    }

    pub async fn start(&self, tenant_id: Uuid, id: Uuid) -> Result<schedule_plan::Model, DbErr> {
        let plan = schedule_plan::Entity::find_by_id(id)
            .filter(schedule_plan::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Schedule plan not found".into()))?;

        let mut plan: schedule_plan::ActiveModel = plan.into();
        plan.status = Set("in_progress".to_string());
        plan.updated_at = Set(chrono::Utc::now().naive_utc());

        plan.update(self.db.as_ref()).await
    }

    pub async fn complete(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<schedule_plan::Model, DbErr> {
        let plan = schedule_plan::Entity::find_by_id(id)
            .filter(schedule_plan::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Schedule plan not found".into()))?;

        let mut plan: schedule_plan::ActiveModel = plan.into();
        plan.status = Set("completed".to_string());
        plan.updated_at = Set(chrono::Utc::now().naive_utc());

        plan.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<schedule_plan::Model>, DbErr> {
        schedule_plan::Entity::find_by_id(id)
            .filter(schedule_plan::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_by_work_order(
        &self,
        tenant_id: Uuid,
        work_order_id: Uuid,
    ) -> Result<Vec<schedule_plan::Model>, DbErr> {
        schedule_plan::Entity::find()
            .filter(schedule_plan::Column::TenantId.eq(tenant_id))
            .filter(schedule_plan::Column::WorkOrderId.eq(work_order_id))
            .order_by_asc(schedule_plan::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<schedule_plan::Model>, DbErr> {
        schedule_plan::Entity::find()
            .filter(schedule_plan::Column::TenantId.eq(tenant_id))
            .order_by_desc(schedule_plan::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        schedule_plan::Entity::delete_by_id(id)
            .filter(schedule_plan::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
