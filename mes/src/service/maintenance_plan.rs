use crate::entity::maintenance_plan;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct MaintenancePlanService {
    db: Arc<DatabaseConnection>,
}

impl MaintenancePlanService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        equipment_id: Uuid,
        plan_type: String,
        plan_date: Option<chrono::NaiveDate>,
        content: Option<String>,
    ) -> Result<maintenance_plan::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let plan = maintenance_plan::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            equipment_id: Set(equipment_id),
            plan_type: Set(plan_type),
            plan_date: Set(plan_date),
            content: Set(content),
            status: Set("pending".to_string()),
            executor_id: Set(None),
            execute_time: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        plan.insert(self.db.as_ref()).await
    }

    pub async fn execute(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        executor_id: Uuid,
        content: Option<String>,
    ) -> Result<maintenance_plan::Model, DbErr> {
        let plan = maintenance_plan::Entity::find_by_id(id)
            .filter(maintenance_plan::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Maintenance plan not found".into()))?;

        let now = chrono::Utc::now().naive_utc();
        let mut plan: maintenance_plan::ActiveModel = plan.into();
        plan.status = Set("completed".to_string());
        plan.executor_id = Set(Some(executor_id));
        plan.execute_time = Set(Some(now));
        if let Some(content) = content {
            plan.content = Set(Some(content));
        }
        plan.updated_at = Set(now);

        plan.update(self.db.as_ref()).await
    }

    pub async fn update_status(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        status: String,
    ) -> Result<maintenance_plan::Model, DbErr> {
        let plan = maintenance_plan::Entity::find_by_id(id)
            .filter(maintenance_plan::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Maintenance plan not found".into()))?;

        let mut plan: maintenance_plan::ActiveModel = plan.into();
        plan.status = Set(status);
        plan.updated_at = Set(chrono::Utc::now().naive_utc());

        plan.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<maintenance_plan::Model>, DbErr> {
        maintenance_plan::Entity::find_by_id(id)
            .filter(maintenance_plan::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_by_equipment(
        &self,
        tenant_id: Uuid,
        equipment_id: Uuid,
    ) -> Result<Vec<maintenance_plan::Model>, DbErr> {
        maintenance_plan::Entity::find()
            .filter(maintenance_plan::Column::TenantId.eq(tenant_id))
            .filter(maintenance_plan::Column::EquipmentId.eq(equipment_id))
            .order_by_desc(maintenance_plan::Column::PlanDate)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_by_status(
        &self,
        tenant_id: Uuid,
        status: String,
    ) -> Result<Vec<maintenance_plan::Model>, DbErr> {
        maintenance_plan::Entity::find()
            .filter(maintenance_plan::Column::TenantId.eq(tenant_id))
            .filter(maintenance_plan::Column::Status.eq(status))
            .order_by_desc(maintenance_plan::Column::PlanDate)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<maintenance_plan::Model>, DbErr> {
        maintenance_plan::Entity::find()
            .filter(maintenance_plan::Column::TenantId.eq(tenant_id))
            .order_by_desc(maintenance_plan::Column::PlanDate)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        maintenance_plan::Entity::delete_by_id(id)
            .filter(maintenance_plan::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
