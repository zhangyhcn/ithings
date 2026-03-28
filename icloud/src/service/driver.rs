use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, PaginatorTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use reqwest::Client;

use crate::entity::{DriverEntity, DriverColumn, DriverModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDriverRequest {
    pub name: String,
    pub description: Option<String>,
    pub protocol_type: String,
    pub image: String,
    pub version: String,
    #[serde(default)]
    pub device_profile: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDriverRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub protocol_type: Option<String>,
    pub image: Option<String>,
    pub version: Option<String>,
    pub device_profile: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverResponse {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub description: Option<String>,
    pub protocol_type: String,
    pub image: String,
    pub version: String,
    pub device_profile: JsonValue,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
struct DockerTagsResponse {
    name: String,
    tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct DockerCatalogResponse {
    repositories: Vec<String>,
}

impl From<Model> for DriverResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            name: model.name,
            description: model.description,
            protocol_type: model.protocol_type.clone(),
            image: model.image.clone(),
            version: model.version.clone(),
            device_profile: model.device_profile,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct DriverService {
    db: DatabaseConnection,
}

impl DriverService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    async fn get_tenant_registry_url(&self, tenant_id: Uuid) -> Result<Option<String>, AppError> {
        let tenant = crate::entity::tenant::Entity::find_by_id(tenant_id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to query tenant: {}", e)))?;

        if let Some(tenant) = tenant {
            if let Some(config) = tenant.config {
                if let Some(url) = config.get("registry_url").and_then(|v| v.as_str()) {
                    return Ok(Some(url.to_string()));
                }
            }
        }
        Ok(None)
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateDriverRequest,
    ) -> Result<DriverResponse, AppError> {
        let tenant = crate::service::tenant::TenantCache::get(tenant_id).await
            .ok_or(AppError::TenantNotFound)?;

        let mut active_model = crate::entity::driver::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            name: Set(req.name),
            description: Set(req.description),
            protocol_type: Set(req.protocol_type),
            image: Set(req.image),
            version: Set(req.version),
            device_profile: Set(req.device_profile.unwrap_or(serde_json::json!({}))),
            ..Default::default()
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<DriverResponse, AppError> {
        let model = DriverEntity::find()
            .filter(DriverColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::NotFound("Driver not found".to_string())),
        }
    }

    pub async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<DriverResponse>, AppError> {
        tracing::debug!("list_by_tenant called with tenant_id: {}", tenant_id);

        if let Err(e) = self.sync_drivers_from_registry(tenant_id).await {
            tracing::warn!("Failed to sync drivers from registry: {}", e);
        }

        let models = DriverEntity::find()
            .filter(DriverColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        tracing::debug!("Found {} drivers for tenant {}", models.len(), tenant_id);

        Ok(models.into_iter().map(Into::into).collect())
    }

    async fn sync_drivers_from_registry(&self, tenant_id: Uuid) -> Result<(), AppError> {
        let registry_url = self.get_tenant_registry_url(tenant_id).await?;

        let Some(registry_url) = registry_url else {
            tracing::info!("No registry URL configured for tenant {}", tenant_id);
            return Ok(());
        };

        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| AppError::InternalServerError(format!("Failed to create HTTP client: {}", e)))?;

        let registry = registry_url.trim_end_matches('/');
        let url = format!("{}/v2/_catalog", registry);

        let res = client.get(&url).send().await;

        let registry_images = match res {
            Ok(response) => {
                if response.status().is_success() {
                    let catalog_resp: DockerCatalogResponse = response.json()
                        .await
                        .map_err(|e| AppError::InternalServerError(format!("Failed to parse response: {}", e)))?;

                    catalog_resp.repositories
                        .into_iter()
                        .filter(|img| img.to_lowercase().contains("driver"))
                        .collect::<Vec<_>>()
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    tracing::warn!("Registry API failed: {} - {}", status.as_u16(), text);
                    return Ok(());
                }
            }
            Err(e) => {
                tracing::warn!("Failed to connect to registry {}: {}", registry_url, e);
                return Ok(());
            }
        };

        tracing::debug!("Found {} driver images in registry", registry_images.len());

        let existing_drivers = DriverEntity::find()
            .filter(DriverColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        let existing_images: std::collections::HashMap<String, Model> = existing_drivers
            .into_iter()
            .map(|d| (d.image.clone(), d))
            .collect();

        for image in registry_images {
            let name = image.split('/').last().unwrap_or(&image).to_string();

            if let Some(existing) = existing_images.get(&image) {
                tracing::debug!("Driver {} already exists in database", image);
            } else {
                tracing::info!("Creating new driver for image: {}", image);
                let active_model = crate::entity::driver::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    tenant_id: Set(tenant_id),
                    name: Set(name),
                    description: Set(Some(format!("Registry image: {}", image))),
                    protocol_type: Set("unknown".to_string()),
                    image: Set(image.clone()),
                    version: Set("latest".to_string()),
                    device_profile: Set(serde_json::json!({})),
                    ..Default::default()
                };
                active_model.insert(&self.db).await?;
            }
        }

        Ok(())
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateDriverRequest,
    ) -> Result<DriverResponse, AppError> {
        let model = DriverEntity::find()
            .filter(DriverColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(mut model) = model else {
            return Err(AppError::NotFound("Driver not found".to_string()));
        };

        let mut active_model = model.into_active_model();

        if let Some(name) = req.name {
            active_model.name = Set(name);
        }
        if let Some(description) = req.description {
            active_model.description = Set(Some(description));
        }
        if let Some(protocol_type) = req.protocol_type {
            active_model.protocol_type = Set(protocol_type);
        }
        if let Some(image) = req.image {
            active_model.image = Set(image);
        }
        if let Some(version) = req.version {
            active_model.version = Set(version);
        }
        if let Some(device_profile) = req.device_profile {
            active_model.device_profile = Set(device_profile);
        }

        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let model = DriverEntity::find()
            .filter(DriverColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(model) = model else {
            return Err(AppError::NotFound("Driver not found".to_string()));
        };

        DriverEntity::delete(model.into_active_model()).exec(&self.db).await?;
        Ok(())
    }

    pub async fn list_image_tags(
        &self,
        tenant_id: Uuid,
        registry: Option<String>,
        image: String,
    ) -> Result<Vec<String>, AppError> {
        let registry_url = if let Some(r) = registry {
            Some(r)
        } else {
            self.get_tenant_registry_url(tenant_id).await?
        };

        let Some(registry_url) = registry_url else {
            return Err(AppError::BadRequest("Tenant has no registry url configured".to_string()));
        };

        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| AppError::InternalServerError(format!("Failed to create HTTP client: {}", e)))?;

        let registry = registry_url.trim_end_matches('/');

        let url = format!("{}/v2/{}/tags/list", registry, image);

        let res = client.get(&url)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to query image tags: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(AppError::InternalServerError(
                format!("Registry API failed: {} - {}", status.as_u16(), text)
            ));
        }

        let tags_resp: DockerTagsResponse = res.json()
            .await
            .map_err(|e| AppError::InternalServerError(format!("Failed to parse response: {}", e)))?;

        Ok(tags_resp.tags)
    }

    pub async fn list_registry_images(&self, tenant_id: Uuid) -> Result<Vec<DriverResponse>, AppError> {
        let registry_url = self.get_tenant_registry_url(tenant_id).await?;

        let Some(registry_url) = registry_url else {
            return Err(AppError::BadRequest("Tenant has no registry url configured".to_string()));
        };

        let client = Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| AppError::InternalServerError(format!("Failed to create HTTP client: {}", e)))?;

        let registry = registry_url.trim_end_matches('/');

        let url = format!("{}/v2/_catalog", registry);

        let res = client.get(&url)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    let catalog_resp: DockerCatalogResponse = response.json()
                        .await
                        .map_err(|e| AppError::InternalServerError(format!("Failed to parse response: {}", e)))?;

                    let drivers: Vec<DriverResponse> = catalog_resp.repositories.into_iter().map(|image| {
                        let name = image.split('/').last().unwrap_or(&image).to_string();
                        DriverResponse {
                            id: format!("registry-{}", image.replace('/', "-")),
                            tenant_id: tenant_id.to_string(),
                            name: name.clone(),
                            description: Some(format!("Registry image: {}", image)),
                            protocol_type: "unknown".to_string(),
                            image,
                            version: "latest".to_string(),
                            device_profile: serde_json::json!({}),
                            created_at: chrono::Utc::now().to_rfc3339(),
                            updated_at: chrono::Utc::now().to_rfc3339(),
                        }
                    }).collect();

                    Ok(drivers)
                } else {
                    let status = response.status();
                    let text = response.text().await.unwrap_or_default();
                    tracing::warn!("Registry API failed: {} - {}", status.as_u16(), text);
                    Ok(vec![])
                }
            }
            Err(e) => {
                tracing::warn!("Failed to connect to registry {}: {}", registry_url, e);
                Ok(vec![])
            }
        }
    }
}