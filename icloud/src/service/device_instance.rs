use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::entity::{DeviceInstanceEntity, DeviceInstanceColumn, DeviceInstanceModel as Model, DeviceGroupEntity, DeviceEntity, DeviceColumn};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceInstanceRequest {
    pub group_id: String,
    pub device_id: String,
    pub name: String,
    pub driver_config: Option<JsonValue>,
    pub thing_model: Option<JsonValue>,
    pub poll_interval_ms: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceInstanceRequest {
    pub name: Option<String>,
    pub driver_config: Option<JsonValue>,
    pub thing_model: Option<JsonValue>,
    pub poll_interval_ms: Option<i32>,
    pub node_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceInstanceResponse {
    pub id: String,
    pub tenant_id: String,
    pub group_id: String,
    pub device_id: String,
    pub product_id: Option<String>,
    pub name: String,
    pub driver_config: JsonValue,
    pub thing_model: JsonValue,
    pub poll_interval_ms: i32,
    pub node_id: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for DeviceInstanceResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            group_id: model.group_id.to_string(),
            device_id: model.device_id.to_string(),
            product_id: model.product_id.map(|id| id.to_string()),
            name: model.name,
            driver_config: model.driver_config,
            thing_model: model.thing_model,
            poll_interval_ms: model.poll_interval_ms,
            node_id: model.node_id.map(|n| n.to_string()),
            status: model.status,
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
        req: CreateDeviceInstanceRequest,
    ) -> Result<DeviceInstanceResponse, AppError> {
        let group_id = Uuid::parse_str(&req.group_id)
            .map_err(|_| AppError::BadRequest("Invalid group_id".to_string()))?;

        let group = DeviceGroupEntity::find_by_id(group_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Device group not found".to_string()))?;

        if group.tenant_id != tenant_id {
            return Err(AppError::BadRequest("Device group does not belong to tenant".to_string()));
        }

        let device_id = Uuid::parse_str(&req.device_id)
            .map_err(|_| AppError::BadRequest("Invalid device_id".to_string()))?;

        let device = DeviceEntity::find_by_id(device_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Device not found".to_string()))?;

        let product_id = device.product_id;

        let driver_config = req.driver_config.unwrap_or_else(|| device.device_profile.clone());
        let thing_model = req.thing_model.unwrap_or_else(|| {
            product_id.map(|p| serde_json::json!({}))
                .unwrap_or_else(|| serde_json::json!({}))
        });

        let now = chrono::Utc::now().naive_utc();
        let active_model = crate::entity::device_instance::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            group_id: Set(group_id),
            device_id: Set(device_id),
            product_id: Set(product_id),
            name: Set(req.name),
            driver_config: Set(driver_config),
            thing_model: Set(thing_model),
            poll_interval_ms: Set(req.poll_interval_ms.unwrap_or(1000)),
            node_id: Set(None),
            status: Set("pending".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<DeviceInstanceResponse, AppError> {
        let model = DeviceInstanceEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Device instance not found".to_string()))?;

        Ok(model.into())
    }

    pub async fn list_by_tenant(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<DeviceInstanceResponse>, AppError> {
        let models = DeviceInstanceEntity::find()
            .filter(DeviceInstanceColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    pub async fn list_by_group(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<DeviceInstanceResponse>, AppError> {
        let models = DeviceInstanceEntity::find()
            .filter(DeviceInstanceColumn::GroupId.eq(group_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateDeviceInstanceRequest,
    ) -> Result<DeviceInstanceResponse, AppError> {
        let mut model = DeviceInstanceEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Device instance not found".to_string()))?
            .into_active_model();

        if let Some(name) = req.name {
            model.name = Set(name);
        }
        if let Some(driver_config) = req.driver_config {
            model.driver_config = Set(driver_config);
        }
        if let Some(thing_model) = req.thing_model {
            model.thing_model = Set(thing_model);
        }
        if let Some(poll_interval_ms) = req.poll_interval_ms {
            model.poll_interval_ms = Set(poll_interval_ms);
        }
        if let Some(node_id_str) = req.node_id {
            let node_id = if node_id_str.is_empty() {
                None
            } else {
                Some(Uuid::parse_str(&node_id_str)
                    .map_err(|_| AppError::BadRequest("Invalid node_id".to_string()))?)
            };
            model.node_id = Set(node_id);
        }
        if let Some(status) = req.status {
            model.status = Set(status);
        }
        model.updated_at = Set(chrono::Utc::now().naive_utc());

        let model = model.update(&self.db).await?;
        Ok(model.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = DeviceInstanceEntity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Device instance not found".to_string()));
        }
        Ok(())
    }
}
