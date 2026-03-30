use crate::entity::inspection_order;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct InspectionOrderService {
    db: Arc<DatabaseConnection>,
}

impl InspectionOrderService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        inspection_type: String,
        work_order_id: Option<Uuid>,
        material_id: Option<Uuid>,
        sample_qty: Option<i32>,
        inspector_id: Option<Uuid>,
    ) -> Result<inspection_order::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();

        // 生成检验单编号
        let inspection_no = format!("QC{}", now.format("%Y%m%d%H%M%S"));

        let order = inspection_order::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            inspection_no: Set(inspection_no),
            inspection_type: Set(inspection_type),
            work_order_id: Set(work_order_id),
            material_id: Set(material_id),
            batch_no: Set(None),
            sample_qty: Set(sample_qty),
            pass_qty: Set(None),
            defect_qty: Set(None),
            result: Set("pending".to_string()),
            inspector_id: Set(inspector_id),
            inspect_time: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        order.insert(self.db.as_ref()).await
    }

    pub async fn submit_result(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        pass_qty: i32,
        defect_qty: i32,
        result: String,
        inspector_id: Uuid,
    ) -> Result<inspection_order::Model, DbErr> {
        let order = inspection_order::Entity::find_by_id(id)
            .filter(inspection_order::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Inspection order not found".into()))?;

        let now = chrono::Utc::now().naive_utc();
        let mut order: inspection_order::ActiveModel = order.into();
        order.pass_qty = Set(Some(pass_qty));
        order.defect_qty = Set(Some(defect_qty));
        order.result = Set(result);
        order.inspector_id = Set(Some(inspector_id));
        order.inspect_time = Set(Some(now));
        order.updated_at = Set(now);

        order.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<inspection_order::Model>, DbErr> {
        inspection_order::Entity::find_by_id(id)
            .filter(inspection_order::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_by_type(
        &self,
        tenant_id: Uuid,
        inspection_type: String,
    ) -> Result<Vec<inspection_order::Model>, DbErr> {
        inspection_order::Entity::find()
            .filter(inspection_order::Column::TenantId.eq(tenant_id))
            .filter(inspection_order::Column::InspectionType.eq(inspection_type))
            .order_by_desc(inspection_order::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_by_work_order(
        &self,
        tenant_id: Uuid,
        work_order_id: Uuid,
    ) -> Result<Vec<inspection_order::Model>, DbErr> {
        inspection_order::Entity::find()
            .filter(inspection_order::Column::TenantId.eq(tenant_id))
            .filter(inspection_order::Column::WorkOrderId.eq(work_order_id))
            .order_by_desc(inspection_order::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<inspection_order::Model>, DbErr> {
        inspection_order::Entity::find()
            .filter(inspection_order::Column::TenantId.eq(tenant_id))
            .order_by_desc(inspection_order::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        inspection_order::Entity::delete_by_id(id)
            .filter(inspection_order::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
