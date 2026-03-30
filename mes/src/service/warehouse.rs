use crate::entity::warehouse;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct WarehouseService {
    db: Arc<DatabaseConnection>,
}

impl WarehouseService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        warehouse_no: String,
        warehouse_name: String,
        warehouse_type: Option<String>,
        location: Option<String>,
        description: Option<String>,
    ) -> Result<warehouse::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let warehouse = warehouse::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            warehouse_no: Set(warehouse_no),
            warehouse_name: Set(warehouse_name),
            warehouse_type: Set(warehouse_type),
            location: Set(location),
            description: Set(description),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        warehouse.insert(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        warehouse_name: Option<String>,
        warehouse_type: Option<String>,
        location: Option<String>,
        description: Option<String>,
        status: Option<String>,
    ) -> Result<warehouse::Model, DbErr> {
        let warehouse = warehouse::Entity::find_by_id(id)
            .filter(warehouse::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Warehouse not found".into()))?;

        let mut warehouse: warehouse::ActiveModel = warehouse.into();
        if let Some(name) = warehouse_name {
            warehouse.warehouse_name = Set(name);
        }
        if let Some(wtype) = warehouse_type {
            warehouse.warehouse_type = Set(Some(wtype));
        }
        if let Some(loc) = location {
            warehouse.location = Set(Some(loc));
        }
        if let Some(desc) = description {
            warehouse.description = Set(Some(desc));
        }
        if let Some(s) = status {
            warehouse.status = Set(s);
        }
        warehouse.updated_at = Set(chrono::Utc::now().naive_utc());

        warehouse.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<warehouse::Model>, DbErr> {
        warehouse::Entity::find_by_id(id)
            .filter(warehouse::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<warehouse::Model>, DbErr> {
        warehouse::Entity::find()
            .filter(warehouse::Column::TenantId.eq(tenant_id))
            .order_by_asc(warehouse::Column::WarehouseNo)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        warehouse::Entity::delete_by_id(id)
            .filter(warehouse::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
