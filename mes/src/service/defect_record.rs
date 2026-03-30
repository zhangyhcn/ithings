use crate::entity::defect_record;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct DefectRecordService {
    db: Arc<DatabaseConnection>,
}

impl DefectRecordService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        inspection_id: Uuid,
        quantity: i32,
        defect_type_id: Option<Uuid>,
        defect_code: Option<String>,
        description: Option<String>,
    ) -> Result<defect_record::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let record = defect_record::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            inspection_id: Set(inspection_id),
            defect_type_id: Set(defect_type_id),
            defect_code: Set(defect_code),
            quantity: Set(quantity),
            description: Set(description),
            disposition: Set("pending".to_string()),
            status: Set("pending".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        record.insert(self.db.as_ref()).await
    }

    pub async fn handle(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        disposition: String,
    ) -> Result<defect_record::Model, DbErr> {
        let record = defect_record::Entity::find_by_id(id)
            .filter(defect_record::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Defect record not found".into()))?;

        let now = chrono::Utc::now().naive_utc();
        let mut record: defect_record::ActiveModel = record.into();
        record.status = Set("processed".to_string());
        record.disposition = Set(disposition);
        record.updated_at = Set(now);

        record.update(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        quantity: Option<i32>,
        description: Option<String>,
        status: Option<String>,
    ) -> Result<defect_record::Model, DbErr> {
        let record = defect_record::Entity::find_by_id(id)
            .filter(defect_record::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Defect record not found".into()))?;

        let mut record: defect_record::ActiveModel = record.into();
        if let Some(qty) = quantity {
            record.quantity = Set(qty);
        }
        if let Some(desc) = description {
            record.description = Set(Some(desc));
        }
        if let Some(s) = status {
            record.status = Set(s);
        }
        record.updated_at = Set(chrono::Utc::now().naive_utc());

        record.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<defect_record::Model>, DbErr> {
        defect_record::Entity::find_by_id(id)
            .filter(defect_record::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_by_inspection(
        &self,
        tenant_id: Uuid,
        inspection_id: Uuid,
    ) -> Result<Vec<defect_record::Model>, DbErr> {
        defect_record::Entity::find()
            .filter(defect_record::Column::TenantId.eq(tenant_id))
            .filter(defect_record::Column::InspectionId.eq(inspection_id))
            .order_by_desc(defect_record::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<defect_record::Model>, DbErr> {
        defect_record::Entity::find()
            .filter(defect_record::Column::TenantId.eq(tenant_id))
            .order_by_desc(defect_record::Column::CreatedAt)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        defect_record::Entity::delete_by_id(id)
            .filter(defect_record::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
