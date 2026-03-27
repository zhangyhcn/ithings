use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{DeviceGroupEntity, DeviceGroupColumn, DeviceGroupModel as Model, TenantEntity, OrganizationEntity, SiteEntity};
use crate::service::cache::GlobalCache;
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceGroupRequest {
    pub org_id: String,
    pub site_id: String,
    pub name: String,
    pub description: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishDeviceGroupRequest {
    pub node_id: Option<String>,
    pub labels: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceGroupResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub site_id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub node_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for DeviceGroupResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            org_id: model.org_id.to_string(),
            site_id: model.site_id.to_string(),
            name: model.name,
            description: model.description,
            status: model.status,
            node_id: model.node_id.map(|id| id.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct DeviceGroupService {
    db: DatabaseConnection,
}

impl DeviceGroupService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateDeviceGroupRequest,
    ) -> Result<DeviceGroupResponse, AppError> {
        let _tenant = TenantEntity::find_by_id(tenant_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::TenantNotFound)?;

        let org_id = Uuid::parse_str(&req.org_id)
            .map_err(|_| AppError::BadRequest("Invalid org_id".to_string()))?;

        let _org = OrganizationEntity::find_by_id(org_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Organization not found".to_string()))?;

        let site_id = Uuid::parse_str(&req.site_id)
            .map_err(|_| AppError::BadRequest("Invalid site_id".to_string()))?;

        let site = SiteEntity::find_by_id(site_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Site not found".to_string()))?;

        if site.organization_id != org_id {
            return Err(AppError::BadRequest("Site does not belong to the organization".to_string()));
        }

        let now = chrono::Utc::now().naive_utc();
        let node_id = req.node_id.and_then(|id| Uuid::parse_str(&id).ok());
        let active_model = crate::entity::device_group::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            site_id: Set(site_id),
            name: Set(req.name),
            description: Set(req.description),
            status: Set("active".to_string()),
            node_id: Set(node_id),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<DeviceGroupResponse, AppError> {
        let model = DeviceGroupEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Device group not found".to_string()))?;

        Ok(model.into())
    }

    pub async fn list_by_tenant(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<DeviceGroupResponse>, AppError> {
        let models = DeviceGroupEntity::find()
            .filter(DeviceGroupColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    pub async fn list_by_site(
        &self,
        site_id: Uuid,
    ) -> Result<Vec<DeviceGroupResponse>, AppError> {
        let models = DeviceGroupEntity::find()
            .filter(DeviceGroupColumn::SiteId.eq(site_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateDeviceGroupRequest,
    ) -> Result<DeviceGroupResponse, AppError> {
        let mut model = DeviceGroupEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Device group not found".to_string()))?
            .into_active_model();

        if let Some(name) = req.name {
            model.name = Set(name);
        }
        if let Some(description) = req.description {
            model.description = Set(Some(description));
        }
        if let Some(status) = req.status {
            model.status = Set(status);
        }
        if let Some(node_id) = req.node_id {
            model.node_id = Set(node_id.parse().ok());
        }
        model.updated_at = Set(chrono::Utc::now().naive_utc());

        let model = model.update(&self.db).await?;
        Ok(model.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = DeviceGroupEntity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Device group not found".to_string()));
        }
        Ok(())
    }

    pub async fn publish(
        &self,
        id: Uuid,
        req: PublishDeviceGroupRequest,
    ) -> Result<(), AppError> {
        let group = DeviceGroupEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Device group not found".to_string()))?;

        let instances = crate::entity::device_instance::Entity::find()
            .filter(crate::entity::device_instance::Column::GroupId.eq(id))
            .all(&self.db)
            .await?;

        let mut instance_configs = Vec::new();
        let mut device_images = std::collections::HashSet::new();
        let mut driver_image: Option<String> = None;

        for inst in &instances {
            let device = crate::entity::device::Entity::find_by_id(inst.device_id)
                .one(&self.db)
                .await?
                .ok_or(AppError::NotFound("Device not found".to_string()))?;

            let product_id = device.product_id
                .ok_or(AppError::BadRequest("Device is not associated with any product".to_string()))?;

            let product = crate::entity::product::Entity::find_by_id(product_id)
                .one(&self.db)
                .await?
                .ok_or(AppError::NotFound("Product not found".to_string()))?;

            let driver_custom_config = inst.driver_config.clone();
             
             let driver_type = device.driver_image.as_ref()
                 .and_then(|s| s.split('/').next())
                 .unwrap_or("modbus")
                 .to_string();

            instance_configs.push(serde_json::json!({
                 "device_id": inst.id.to_string(),
                 "device_name": inst.name.clone(),
                 "device_type": product.name.clone(),
                 "poll_interval_ms": inst.poll_interval_ms,
                 "driver": {
                     "driver_name": device.driver_image.as_ref().map(|s| s.replace("driver/", "").replace(":latest", "")).unwrap_or_else(|| "modbus-driver".to_string()),
                     "driver_type": driver_type,
                     "poll_interval_ms": inst.poll_interval_ms,
                     "zmq": {
                         "enabled": true,
                         "publisher_address": "tcp://127.0.0.1:5556",
                         "topic": "driver/data"
                     },
                     "logging": {
                         "level": "info",
                         "format": "json"
                     },
                     "custom": driver_custom_config
                 },
                 "thing_model": {
                     "model_id": product.id.to_string(),
                     "model_version": "1.0",
                     "device_type": product.name.clone(),
                     "manufacturer": "ithings",
                     "description": product.description.unwrap_or_default(),
                     "properties": product.thing_model.get("properties").cloned().unwrap_or(serde_json::json!([])),
                     "events": product.thing_model.get("events").cloned().unwrap_or(serde_json::json!([])),
                     "services": product.thing_model.get("services").cloned().unwrap_or(serde_json::json!([]))
                 }
             }));

            let device_image = format_image_with_registry(group.tenant_id, &device.device_image).await;
            device_images.insert(device_image);
            if let Some(drv_img) = device.driver_image {
                let driver_image_full = format_image_with_registry(group.tenant_id, &drv_img).await;
                driver_image = Some(driver_image_full);
            }
        }

        if instance_configs.is_empty() {
            return Err(AppError::BadRequest("No device instances in this device group".to_string()));
        }

        let driver_image = driver_image.ok_or(AppError::BadRequest("No driver image configured in any device definition".to_string()))?;

        let labels = req.labels.unwrap_or_default();

        let deployment_name = format!("device-group-{}", id);
        
        let k8s_client = crate::k8s::K8sClient::new(None).await?;
        k8s_client
            .create_or_update_deployment(
                &deployment_name,
                &device_images.into_iter().collect::<Vec<_>>(),
                &driver_image,
                &labels,
                &instance_configs,
                id,
            )
            .await?;

        let mut model = group.into_active_model();
        model.status = Set("published".to_string());
        model.updated_at = Set(chrono::Utc::now().naive_utc());
        model.update(&self.db).await?;

        tracing::info!("Successfully published device group: {}", id);
        Ok(())
    }
}

/// Format image name with registry prefix from tenant config
async fn format_image_with_registry(tenant_id: Uuid, image: &str) -> String {
    // Check if image already contains a registry domain
    // If there's a dot before the first slash, it's a domain (registry address)
    // If no dot before first slash, it's on Docker Hub, we need to prepend our registry
    let has_domain = image.find('/')
        .map(|first_slash| image[..first_slash].contains('.'))
        .unwrap_or(false);
    
    if has_domain {
        return image.to_string();
    }
    if !has_domain && image.contains('/') {
        // No domain but has slash, still need to prepend registry
        // example: ithings/modbus-driver → 127.0.0.1:30500/ithings/modbus-driver
    } else {
        // No slash at all, just name: device-meter → 127.0.0.1:30500/device-meter
    }
    
    // Need to prepend registry
    if let Some(tenant) = crate::service::cache::GlobalCache::get_tenant(tenant_id).await {
        if let Some(config) = tenant.config {
            if let serde_json::Value::Object(obj) = config {
                if let Some(registry) = obj.get("registry_url") {
                    if let Some(mut registry_str) = registry.as_str().map(|s| s.to_string()) {
                        // Remove scheme prefix (http:// or https://) if present
                        registry_str = registry_str
                            .replace("http://", "")
                            .replace("https://", "");
                        let registry: &str = &registry_str.trim_end_matches('/');
                        return format!("{registry}/{image}");
                    }
                }
            }
        }
    }
    image.to_string()
}
