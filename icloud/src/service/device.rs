use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use chrono::Utc;

use crate::entity::{DeviceEntity, DeviceColumn, DeviceModel as Model};
use crate::entity::tenant;
use crate::utils::AppError;
use device_common::config::driver::DriverConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceRequest {
    pub name: String,
    pub product_id: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub device_image: String,
    pub driver_image: Option<String>,
    #[serde(default)]
    pub device_profile: Option<JsonValue>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceRequest {
    pub name: Option<String>,
    pub product_id: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub device_image: Option<String>,
    pub driver_image: Option<String>,
    pub device_profile: Option<JsonValue>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceResponse {
    pub id: String,
    pub tenant_id: String,
    pub product_id: Option<String>,
    pub name: String,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub device_image: String,
    pub driver_image: Option<String>,
    pub device_profile: JsonValue,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for DeviceResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            product_id: model.product_id.map(|id| id.to_string()),
            name: model.name,
            model: model.model,
            manufacturer: model.manufacturer,
            device_image: model.device_image,
            driver_image: model.driver_image,
            device_profile: model.device_profile,
            description: model.description,
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct DeviceService {
    db: DatabaseConnection,
}

impl DeviceService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    fn validate_device_profile(device_profile: &JsonValue) -> Result<(), AppError> {
        if device_profile.is_null() || device_profile.as_object().map_or(true, |obj| obj.is_empty()) {
            return Ok(());
        }

        let _: DriverConfig = serde_json::from_value(device_profile.clone())
            .map_err(|e| AppError::Validation(format!("设备配置文件格式错误: {}", e)))?;

        Ok(())
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateDeviceRequest,
    ) -> Result<DeviceResponse, AppError> {
        let tenant = tenant::Entity::find()
            .filter(tenant::Column::Id.eq(tenant_id))
            .one(&self.db)
            .await?;

        if tenant.is_none() {
            return Err(AppError::TenantNotFound);
        }

        if let Some(ref device_profile) = req.device_profile {
            Self::validate_device_profile(device_profile)?;
        }

        let now = Utc::now().naive_utc();
        let mut active_model = crate::entity::device::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            product_id: Set(req.product_id.and_then(|id| Uuid::parse_str(&id).ok())),
            name: Set(req.name),
            model: Set(req.model),
            manufacturer: Set(req.manufacturer),
            device_image: Set(req.device_image),
            driver_image: Set(req.driver_image),
            device_profile: Set(req.device_profile.unwrap_or(serde_json::json!({}))),
            description: Set(req.description),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<DeviceResponse, AppError> {
        let model = DeviceEntity::find()
            .filter(DeviceColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::NotFound("Device not found".to_string())),
        }
    }

    pub async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<DeviceResponse>, AppError> {
        let models = DeviceEntity::find()
            .filter(DeviceColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateDeviceRequest,
    ) -> Result<DeviceResponse, AppError> {
        let model = DeviceEntity::find()
            .filter(DeviceColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(mut model) = model else {
            return Err(AppError::NotFound("Device not found".to_string()));
        };

        if let Some(ref device_profile) = req.device_profile {
            Self::validate_device_profile(device_profile)?;
        }

        let mut active_model = model.into_active_model();

        if let Some(name) = req.name {
            active_model.name = Set(name);
        }
        if let Some(product_id) = req.product_id {
            active_model.product_id = Set(Uuid::parse_str(&product_id).ok());
        }
        if let Some(model_val) = req.model {
            active_model.model = Set(Some(model_val));
        }
        if let Some(manufacturer) = req.manufacturer {
            active_model.manufacturer = Set(Some(manufacturer));
        }
        if let Some(device_image) = req.device_image {
            active_model.device_image = Set(device_image);
        }
        if let Some(driver_image) = req.driver_image {
            active_model.driver_image = Set(Some(driver_image));
        }
        if let Some(device_profile) = req.device_profile {
            active_model.device_profile = Set(device_profile);
        }
        if let Some(description) = req.description {
            active_model.description = Set(Some(description));
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }

        active_model.updated_at = Set(Utc::now().naive_utc());

        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let model = DeviceEntity::find()
            .filter(DeviceColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(model) = model else {
            return Err(AppError::NotFound("Device not found".to_string()));
        };

        DeviceEntity::delete(model.into_active_model()).exec(&self.db).await?;
        Ok(())
    }
}
