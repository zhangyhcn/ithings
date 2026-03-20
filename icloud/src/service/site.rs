use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::entity::{SiteEntity, SiteColumn, SiteModel as Model, TenantEntity, OrganizationEntity, OrganizationColumn};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSiteRequest {
    pub organization_id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSiteRequest {
    pub organization_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub config: Option<JsonValue>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteResponse {
    pub id: String,
    pub tenant_id: String,
    pub organization_id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub config: JsonValue,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for SiteResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            organization_id: model.organization_id.to_string(),
            name: model.name,
            slug: model.slug,
            description: model.description,
            location: model.location,
            config: model.config,
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct SiteService {
    db: DatabaseConnection,
}

impl SiteService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, tenant_id: Uuid, req: CreateSiteRequest) -> Result<SiteResponse, AppError> {
        let _tenant = TenantEntity::find_by_id(tenant_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::TenantNotFound)?;

        let organization_id = Uuid::parse_str(&req.organization_id)
            .map_err(|_| AppError::BadRequest("Invalid organization_id".to_string()))?;

        let _org = OrganizationEntity::find_by_id(organization_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Organization not found".to_string()))?;

        let existing = SiteEntity::find()
            .filter(SiteColumn::TenantId.eq(tenant_id))
            .filter(SiteColumn::Slug.eq(&req.slug))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::BadRequest("Site with this slug already exists".to_string()));
        }

        let now = chrono::Utc::now().naive_utc();
        let active_model = crate::entity::site::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            organization_id: Set(organization_id),
            name: Set(req.name),
            slug: Set(req.slug),
            description: Set(req.description),
            location: Set(req.location),
            config: Set(JsonValue::Object(serde_json::Map::new())),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<SiteResponse, AppError> {
        let model = SiteEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Site not found".to_string()))?;

        Ok(model.into())
    }

    pub async fn list_by_tenant(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<SiteResponse>, AppError> {
        let models = SiteEntity::find()
            .filter(SiteColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    pub async fn list_by_organization(
        &self,
        organization_id: Uuid,
    ) -> Result<Vec<SiteResponse>, AppError> {
        let models = SiteEntity::find()
            .filter(SiteColumn::OrganizationId.eq(organization_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateSiteRequest,
    ) -> Result<SiteResponse, AppError> {
        let mut model = SiteEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound("Site not found".to_string()))?
            .into_active_model();

        if let Some(organization_id_str) = req.organization_id {
            let organization_id = Uuid::parse_str(&organization_id_str)
                .map_err(|_| AppError::BadRequest("Invalid organization_id".to_string()))?;
            
            let _org = OrganizationEntity::find_by_id(organization_id)
                .one(&self.db)
                .await?
                .ok_or(AppError::NotFound("Organization not found".to_string()))?;
            
            model.organization_id = Set(organization_id);
        }
        if let Some(name) = req.name {
            model.name = Set(name);
        }
        if let Some(description) = req.description {
            model.description = Set(Some(description));
        }
        if let Some(location) = req.location {
            model.location = Set(Some(location));
        }
        if let Some(config) = req.config {
            model.config = Set(config);
        }
        if let Some(status) = req.status {
            model.status = Set(status);
        }
        model.updated_at = Set(chrono::Utc::now().naive_utc());

        let model = model.update(&self.db).await?;
        Ok(model.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = SiteEntity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Site not found".to_string()));
        }
        Ok(())
    }
}
