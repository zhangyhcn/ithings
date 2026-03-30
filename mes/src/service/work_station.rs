use crate::entity::work_station;
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct WorkStationService {
    db: Arc<DatabaseConnection>,
}

impl WorkStationService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        station_no: String,
        station_name: String,
        workshop_id: Option<Uuid>,
        production_line_id: Option<Uuid>,
        equipment_id: Option<Uuid>,
    ) -> Result<work_station::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let station = work_station::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            station_no: Set(station_no),
            station_name: Set(station_name),
            workshop_id: Set(workshop_id),
            production_line_id: Set(production_line_id),
            equipment_id: Set(equipment_id),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        station.insert(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        station_name: Option<String>,
        workshop_id: Option<Uuid>,
        production_line_id: Option<Uuid>,
        equipment_id: Option<Uuid>,
        status: Option<String>,
    ) -> Result<work_station::Model, DbErr> {
        let station = work_station::Entity::find_by_id(id)
            .filter(work_station::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Work station not found".into()))?;

        let mut station: work_station::ActiveModel = station.into();
        if let Some(name) = station_name {
            station.station_name = Set(name);
        }
        if let Some(wid) = workshop_id {
            station.workshop_id = Set(Some(wid));
        }
        if let Some(lid) = production_line_id {
            station.production_line_id = Set(Some(lid));
        }
        if let Some(eid) = equipment_id {
            station.equipment_id = Set(Some(eid));
        }
        if let Some(s) = status {
            station.status = Set(s);
        }
        station.updated_at = Set(chrono::Utc::now().naive_utc());

        station.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<work_station::Model>, DbErr> {
        work_station::Entity::find_by_id(id)
            .filter(work_station::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_by_workshop(&self, tenant_id: Uuid, workshop_id: Uuid) -> Result<Vec<work_station::Model>, DbErr> {
        work_station::Entity::find()
            .filter(work_station::Column::TenantId.eq(tenant_id))
            .filter(work_station::Column::WorkshopId.eq(workshop_id))
            .order_by_asc(work_station::Column::StationNo)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_by_production_line(&self, tenant_id: Uuid, production_line_id: Uuid) -> Result<Vec<work_station::Model>, DbErr> {
        work_station::Entity::find()
            .filter(work_station::Column::TenantId.eq(tenant_id))
            .filter(work_station::Column::ProductionLineId.eq(production_line_id))
            .order_by_asc(work_station::Column::StationNo)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<work_station::Model>, DbErr> {
        work_station::Entity::find()
            .filter(work_station::Column::TenantId.eq(tenant_id))
            .order_by_asc(work_station::Column::StationNo)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        work_station::Entity::delete_by_id(id)
            .filter(work_station::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
