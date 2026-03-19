use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, PaginatorTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use k8s_openapi::api::core::v1 as core;
use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;
use kube::api::{PostParams, Patch, PatchParams, DeleteParams};
use base64::Engine;

use crate::entity::{SecretEntity, SecretColumn, SecretModel as Model};
use crate::utils::{AppError, K8sClient};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSecretRequest {
    pub namespace_id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub data: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSecretRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub data: Option<JsonValue>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecretResponse {
    pub id: String,
    pub namespace_id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub data: Option<JsonValue>,
    pub status: String,
    pub k8s_name: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for SecretResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            namespace_id: model.namespace_id.to_string(),
            name: model.name,
            slug: model.slug,
            description: model.description,
            data: model.data,
            status: model.status,
            k8s_name: model.k8s_name,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct SecretService {
    db: DatabaseConnection,
}

impl SecretService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, req: CreateSecretRequest) -> Result<SecretResponse, AppError> {
        let namespace_id = Uuid::parse_str(&req.namespace_id)
            .map_err(|e| AppError::BadRequest(format!("Invalid namespace ID: {}", e.to_string())))?;

        let existing = SecretEntity::find()
            .filter(SecretColumn::NamespaceId.eq(namespace_id))
            .filter(SecretColumn::Slug.eq(&req.slug))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::BadRequest("Secret with this slug already exists in the namespace".to_string()));
        }

        let now = chrono::Utc::now().naive_utc();
        let active_model = crate::entity::secret::ActiveModel {
            id: Set(Uuid::new_v4()),
            namespace_id: Set(namespace_id),
            name: Set(req.name),
            slug: Set(req.slug),
            description: Set(req.description),
            data: Set(req.data),
            status: Set("draft".to_string()),
            k8s_name: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let secret = active_model.insert(&self.db).await?;
        Ok(secret.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<SecretResponse, AppError> {
        let model = SecretEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Secret not found".to_string()))?;

        Ok(model.into())
    }

    pub async fn list_by_namespace(
        &self,
        namespace_id: Uuid,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<SecretResponse>, i64), AppError> {
        let page = if page < 1 { 1 } else { page };
        let page_size = if page_size < 1 { 10 } else { page_size };

        let paginator = SecretEntity::find()
            .filter(SecretColumn::NamespaceId.eq(namespace_id))
            .paginate(&self.db, page_size as u64);
        let total = paginator.num_items().await? as i64;
        let models: Vec<Model> = paginator.fetch().await?;

        Ok((models.into_iter().map(|m| m.into()).collect(), total))
    }

    pub async fn list(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<SecretResponse>, i64), AppError> {
        let page = if page < 1 { 1 } else { page };
        let page_size = if page_size < 1 { 10 } else { page_size };

        let paginator = SecretEntity::find().paginate(&self.db, page_size as u64);
        let total = paginator.num_items().await? as i64;
        let models: Vec<Model> = paginator.fetch().await?;

        Ok((models.into_iter().map(|m| m.into()).collect(), total))
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateSecretRequest,
    ) -> Result<SecretResponse, AppError> {
        let mut model = SecretEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Secret not found".to_string()))?
            .into_active_model();

        if let Some(name) = req.name {
            model.name = Set(name);
        }
        if let Some(description) = req.description {
            model.description = Set(Some(description));
        }
        if let Some(data) = req.data {
            model.data = Set(Some(data));
        }
        if let Some(status) = req.status {
            model.status = Set(status);
        }
        model.updated_at = Set(chrono::Utc::now().naive_utc());

        let model = model.update(&self.db).await?;
        Ok(model.into())
    }

    pub async fn publish(&self, id: Uuid) -> Result<SecretResponse, AppError> {
        let model = SecretEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Secret not found".to_string()))?;

        let data = model.data.clone()
            .ok_or(AppError::BadRequest("Secret data is required".to_string()))?;

        let namespace = crate::entity::NamespaceEntity::find_by_id(model.namespace_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Namespace not found".to_string()))?;

        let k8s_secret = self.build_secret_from_data(&data, &namespace.name)?;

        let k8s_client = K8sClient::new().await
            .map_err(|e| AppError::InternalServerError(format!("Failed to create K8s client: {}", e)))?;

        let secret_name = k8s_secret.metadata.name.clone().unwrap_or_default();

        let api: kube::Api<core::Secret> = kube::Api::namespaced(
            k8s_client.client().clone(),
            &namespace.name
        );

        let pp = PostParams::default();
        match api.create(&pp, &k8s_secret).await {
            Ok(secret) => {
                let mut active_model = model.into_active_model();
                active_model.status = Set("published".to_string());
                active_model.k8s_name = Set(Some(secret_name));
                active_model.updated_at = Set(chrono::Utc::now().naive_utc());
                let updated = active_model.update(&self.db).await?;
                Ok(updated.into())
            }
            Err(kube::Error::Api(kube::error::ErrorResponse { code: 409, .. })) => {
                let patch = Patch::Apply(&k8s_secret);
                let pp = PatchParams::apply("icloud");
                match api.patch(&secret_name, &pp, &patch).await {
                    Ok(_) => {
                        let mut active_model = model.into_active_model();
                        active_model.status = Set("published".to_string());
                        active_model.k8s_name = Set(Some(secret_name));
                        active_model.updated_at = Set(chrono::Utc::now().naive_utc());
                        let updated = active_model.update(&self.db).await?;
                        Ok(updated.into())
                    }
                    Err(e) => {
                        Err(AppError::BadRequest(format!("Failed to apply Secret to K8s: {}", e)))
                    }
                }
            }
            Err(e) => {
                Err(AppError::BadRequest(format!("Failed to publish Secret to K8s: {}", e)))
            }
        }
    }

    fn build_secret_from_data(
        &self,
        data: &JsonValue,
        namespace: &str,
    ) -> Result<core::Secret, AppError> {
        let mut secret = core::Secret {
            metadata: metav1::ObjectMeta {
                namespace: Some(namespace.to_string()),
                ..Default::default()
            },
            type_: Some("Opaque".to_string()),
            ..Default::default()
        };

        if let Some(obj) = data.as_object() {
            let mut data_map = std::collections::BTreeMap::new();
            for (key, value) in obj {
                let value_str = if let Some(value_str) = value.as_str() {
                    base64::engine::general_purpose::STANDARD.encode(value_str.as_bytes())
                } else {
                    let plain_text = serde_json::to_string(value)
                        .map_err(|e| AppError::BadRequest(format!("Invalid data format: {}", e)))?;
                    base64::engine::general_purpose::STANDARD.encode(plain_text.as_bytes())
                };
                data_map.insert(key.clone(), k8s_openapi::ByteString(value_str.into_bytes()));
            }
            secret.data = Some(data_map);
        }

        if let Some(name) = data.get("name").and_then(|n| n.as_str()) {
            secret.metadata.name = Some(name.to_string());
        }

        Ok(secret)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let model = SecretEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Secret not found".to_string()))?;

        if let Some(k8s_name) = &model.k8s_name {
            let namespace = crate::entity::NamespaceEntity::find_by_id(model.namespace_id)
                .one(&self.db)
                .await?;

            if let Some(namespace) = namespace {
                match K8sClient::new().await {
                    Ok(k8s_client) => {
                        let api: kube::Api<core::Secret> = kube::Api::namespaced(
                            k8s_client.client().clone(),
                            &namespace.name
                        );
                        if let Err(e) = api.delete(k8s_name, &DeleteParams::default()).await {
                            tracing::warn!("Failed to delete Secret from K8s: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to create K8s client: {}", e);
                    }
                }
            }
        }

        let result = SecretEntity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Secret not found".to_string()));
        }
        Ok(())
    }
}
