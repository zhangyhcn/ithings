use crate::entity::production_line;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct ProductionLineService {
    db: Arc<DatabaseConnection>,
}

impl ProductionLineService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        workshop_id: Option<Uuid>,
        line_no: String,
        line_name: String,
        description: Option<String>,
    ) -> Result<production_line::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let line = production_line::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            workshop_id: Set(workshop_id),
            line_no: Set(line_no),
            line_name: Set(line_name),
            description: Set(description),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        line.insert(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        workshop_id: Option<Uuid>,
        line_name: Option<String>,
        description: Option<String>,
        status: Option<String>,
    ) -> Result<production_line::Model, DbErr> {
        let line = production_line::Entity::find_by_id(id)
            .filter(production_line::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Production line not found".into()))?;

        let mut line: production_line::ActiveModel = line.into();
        if let Some(wid) = workshop_id {
            line.workshop_id = Set(Some(wid));
        }
        if let Some(name) = line_name {
            line.line_name = Set(name);
        }
        if let Some(desc) = description {
            line.description = Set(Some(desc));
        }
        if let Some(s) = status {
            line.status = Set(s);
        }
        line.updated_at = Set(chrono::Utc::now().naive_utc());

        line.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<production_line::Model>, DbErr> {
        production_line::Entity::find_by_id(id)
            .filter(production_line::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_by_workshop(&self, tenant_id: Uuid, workshop_id: Uuid) -> Result<Vec<production_line::Model>, DbErr> {
        production_line::Entity::find()
            .filter(production_line::Column::TenantId.eq(tenant_id))
            .filter(production_line::Column::WorkshopId.eq(workshop_id))
            .order_by_asc(production_line::Column::LineNo)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<production_line::Model>, DbErr> {
        production_line::Entity::find()
            .filter(production_line::Column::TenantId.eq(tenant_id))
            .order_by_asc(production_line::Column::LineNo)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        production_line::Entity::delete_by_id(id)
            .filter(production_line::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
