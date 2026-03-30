use crate::entity::process;
use sea_orm::*;
use serde_json::Value as JsonValue;
use std::sync::Arc;
use uuid::Uuid;

pub struct ProcessService {
    db: Arc<DatabaseConnection>,
}

impl ProcessService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        route_id: Uuid,
        process_no: String,
        process_name: String,
        sequence: i32,
        work_station_id: Option<Uuid>,
        standard_time: Option<rust_decimal::Decimal>,
        setup_time: Option<rust_decimal::Decimal>,
        process_params: Option<JsonValue>,
        next_process_id: Option<Uuid>,
    ) -> Result<process::Model, DbErr> {
        let now = chrono::Utc::now().naive_utc();
        let process = process::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            route_id: Set(route_id),
            process_no: Set(process_no),
            process_name: Set(process_name),
            sequence: Set(sequence),
            work_station_id: Set(work_station_id),
            standard_time: Set(standard_time),
            setup_time: Set(setup_time),
            process_params: Set(process_params),
            next_process_id: Set(next_process_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        process.insert(self.db.as_ref()).await
    }

    pub async fn update(
        &self,
        tenant_id: Uuid,
        id: Uuid,
        process_name: Option<String>,
        sequence: Option<i32>,
        work_station_id: Option<Uuid>,
        standard_time: Option<rust_decimal::Decimal>,
        setup_time: Option<rust_decimal::Decimal>,
        process_params: Option<JsonValue>,
        next_process_id: Option<Uuid>,
    ) -> Result<process::Model, DbErr> {
        let process = process::Entity::find_by_id(id)
            .filter(process::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await?
            .ok_or(DbErr::RecordNotFound("Process not found".into()))?;

        let mut process: process::ActiveModel = process.into();
        if let Some(name) = process_name {
            process.process_name = Set(name);
        }
        if let Some(seq) = sequence {
            process.sequence = Set(seq);
        }
        if let Some(wsid) = work_station_id {
            process.work_station_id = Set(Some(wsid));
        }
        if let Some(st) = standard_time {
            process.standard_time = Set(Some(st));
        }
        if let Some(sut) = setup_time {
            process.setup_time = Set(Some(sut));
        }
        if let Some(params) = process_params {
            process.process_params = Set(Some(params));
        }
        if let Some(npid) = next_process_id {
            process.next_process_id = Set(Some(npid));
        }
        process.updated_at = Set(chrono::Utc::now().naive_utc());

        process.update(self.db.as_ref()).await
    }

    pub async fn get_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<process::Model>, DbErr> {
        process::Entity::find_by_id(id)
            .filter(process::Column::TenantId.eq(tenant_id))
            .one(self.db.as_ref())
            .await
    }

    pub async fn list_by_route(&self, tenant_id: Uuid, route_id: Uuid) -> Result<Vec<process::Model>, DbErr> {
        process::Entity::find()
            .filter(process::Column::TenantId.eq(tenant_id))
            .filter(process::Column::RouteId.eq(route_id))
            .order_by_asc(process::Column::Sequence)
            .all(self.db.as_ref())
            .await
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<process::Model>, DbErr> {
        process::Entity::find()
            .filter(process::Column::TenantId.eq(tenant_id))
            .order_by_asc(process::Column::Sequence)
            .all(self.db.as_ref())
            .await
    }

    pub async fn delete(&self, tenant_id: Uuid, id: Uuid) -> Result<(), DbErr> {
        process::Entity::delete_by_id(id)
            .filter(process::Column::TenantId.eq(tenant_id))
            .exec(self.db.as_ref())
            .await?;
        Ok(())
    }
}
