use crate::entity::location;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct LocationService {
    db: Arc<DatabaseConnection>,
}

impl LocationService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        warehouse_id: Uuid,
        location_no: String,
        location_name: String,
        location_type: Option<String>,
        description: Option<String>,
    ) -> Result<location::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let location = location::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            warehouse_id: Set(warehouse_id),
            location_no: Set(location_no),
            location_name: Set(location_name),
            location_type: Set(location_type),
            description: Set(description),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        location.insert(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        location_name: Option<String>,
        location_type: Option<String>,
        description: Option<String>,
        status: Option<String>,
    ) -> Result<location::Model, DbErr> {
        let location = location::Entity::find_by_id(id)
            .filter(location::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Location not found".into()))?;

        let mut location: location::ActiveModel = location.into();
        if let Some(name) = location_name {
            location.location_name = Set(name);
        }
        if let Some(ltype) = location_type {
            location.location_type = Set(Some(ltype));
        }
        if let Some(desc) = description {
            location.description = Set(Some(desc));
        }
        if let Some(s) = status {
            location.status = Set(s);
        }
        location.updated_at = Set(chrono::Utc::now().naive_utc());

        location.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<location::Model>, DbErr> {
        location::Entity::find_by_id(id)
            .filter(location::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_by_warehouse(&self, tenant_id: Uuid, warehouse_id: Uuid) -> Result<Vec<location::Model>, DbErr> {
        location::Entity::find()
            .filter(location::Column::TenantId.eq(tenant_id))
            .filter(location::Column::WarehouseId.eq(warehouse_id))
            .order_by_asc(location::Column::LocationNo)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<location::Model>, DbErr> {
        location::Entity::find()
            .filter(location::Column::TenantId.eq(tenant_id))
            .order_by_asc(location::Column::LocationNo)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        location::Entity::delete_by_id(id)
            .filter(location::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
