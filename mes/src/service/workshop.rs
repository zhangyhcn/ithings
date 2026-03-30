use crate::entity::workshop;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct WorkshopService {
    db: Arc<DatabaseConnection>,
}

impl WorkshopService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        workshop_no: String,
        workshop_name: String,
        location: Option<String>,
        description: Option<String>,
    ) -> Result<workshop::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let workshop = workshop::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            workshop_no: Set(workshop_no),
            workshop_name: Set(workshop_name),
            location: Set(location),
            description: Set(description),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        workshop.insert(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        workshop_name: Option<String>,
        location: Option<String>,
        description: Option<String>,
        status: Option<String>,
    ) -> Result<workshop::Model, DbErr> {
        let workshop = workshop::Entity::find_by_id(id)
            .filter(workshop::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Workshop not found".into()))?;

        let mut workshop: workshop::ActiveModel = workshop.into();
        if let Some(name) = workshop_name {
            workshop.workshop_name = Set(name);
        }
        if let Some(loc) = location {
            workshop.location = Set(Some(loc));
        }
        if let Some(desc) = description {
            workshop.description = Set(Some(desc));
        }
        if let Some(s) = status {
            workshop.status = Set(s);
        }
        workshop.updated_at = Set(chrono::Utc::now().naive_utc());

        workshop.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<workshop::Model>, DbErr> {
        workshop::Entity::find_by_id(id)
            .filter(workshop::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<workshop::Model>, DbErr> {
        workshop::Entity::find()
            .filter(workshop::Column::TenantId.eq(tenant_id))
            .order_by_asc(workshop::Column::WorkshopNo)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        workshop::Entity::delete_by_id(id)
            .filter(workshop::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
