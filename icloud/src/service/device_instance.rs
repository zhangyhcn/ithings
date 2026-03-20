use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::entity::{DeviceInstanceEntity, DeviceInstanceColumn, DeviceInstanceModel as Model};
use crate::entity::{site, tenant, organization};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceInstanceRequest {
    pub device_id: Uuid,
    pub poll_interval_ms: u64,
    pub driver_config: Option<JsonValue>,
    pub thing_model: Option<JsonValue>,
    pub node_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceInstanceRequest {
    pub device_id: Option<Uuid>,
    pub poll_interval_ms: Option<u64>,
    pub driver_config: Option<JsonValue>,
    pub thing_model: Option<JsonValue>,
    pub node_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInstanceResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub site_id: String,
    pub device_id: String,
    pub poll_interval_ms: u64,
    pub driver_config: JsonValue,
    pub thing_model: JsonValue,
    pub node_id: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for DeviceInstanceResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            site_id: model.site_id.to_string(),
            device_id: model.device_id.to_string(),
            poll_interval_ms: model.poll_interval_ms,
            driver_config: model.driver_config,
            thing_model: model.thing_model,
            node_id: model.node_id.to_string(),
            status: model.status.clone(),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct DeviceInstanceService {
    db: DatabaseConnection,
}

impl DeviceInstanceService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        site_id: Uuid,
        req: CreateDeviceInstanceRequest,
    ) -> Result<DeviceInstanceResponse, AppError> {
        let tenant = tenant::Entity::find()
            .filter(tenant::Column::Id.eq(tenant_id))
            .one(&self.db)
            .await?;

        if tenant.is_none() {
            return Err(AppError::TenantNotFound);
        }

        let org = organization::Entity::find()
            .filter(organization::Column::Id.eq(org_id))
            .one(&self.db)
            .await?;

        if org.is_none() {
            return Err(AppError::NotFound("Organization not found".to_string()));
        }

        let site = site::Entity::find()
            .filter(site::Column::Id.eq(site_id))
            .one(&self.db)
            .await?;

        if site.is_none() {
            return Err(AppError::NotFound("Site not found".to_string()));
        }

        let mut active_model = crate::entity::device_instance::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            site_id: Set(site_id),
            device_id: Set(req.device_id),
            poll_interval_ms: Set(req.poll_interval_ms),
            driver_config: Set(req.driver_config.unwrap_or(serde_json::json!({}))),
            thing_model: Set(req.thing_model.unwrap_or(serde_json::json!({}))),
            node_id: Set(req.node_id),
            status: Set("pending".to_string()),
            ..Default::default()
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<DeviceInstanceResponse, AppError> {
        let model = DeviceInstanceEntity::find()
            .filter(DeviceInstanceColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::NotFound("Device instance not found".to_string())),
        }
    }

    pub async fn list_by_site(
        &self,
        tenant_id: Uuid,
        org_id: Uuid,
        site_id: Uuid,
    ) -> Result<Vec<DeviceInstanceResponse>, AppError> {
        let models = DeviceInstanceEntity::find()
            .filter(DeviceInstanceColumn::TenantId.eq(tenant_id))
            .filter(DeviceInstanceColumn::OrgId.eq(org_id))
            .filter(DeviceInstanceColumn::SiteId.eq(site_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateDeviceInstanceRequest,
    ) -> Result<DeviceInstanceResponse, AppError> {
        let model = DeviceInstanceEntity::find()
            .filter(DeviceInstanceColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(mut model) = model else {
            return Err(AppError::NotFound("Device instance not found".to_string()));
        };

        let mut active_model = model.into_active_model();

        if let Some(device_id) = req.device_id {
            active_model.device_id = Set(device_id);
        }
        if let Some(poll_interval_ms) = req.poll_interval_ms {
            active_model.poll_interval_ms = Set(poll_interval_ms);
        }
        if let Some(driver_config) = req.driver_config {
            active_model.driver_config = Set(driver_config);
        }
        if let Some(thing_model) = req.thing_model {
            active_model.thing_model = Set(thing_model);
        }
        if let Some(node_id) = req.node_id {
            active_model.node_id = Set(node_id);
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }

        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let model = DeviceInstanceEntity::find()
            .filter(DeviceInstanceColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(model) = model else {
            return Err(AppError::NotFound("Device instance not found".to_string()));
        };

        DeviceInstanceEntity::delete(model.into_active_model()).exec(&self.db).await?;
        Ok(())
    }
}
