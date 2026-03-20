use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{DeviceGroupEntity, DeviceGroupColumn, DeviceGroupModel as Model, TenantEntity, OrganizationEntity, SiteEntity};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDeviceGroupRequest {
    pub org_id: String,
    pub site_id: String,
    pub name: String,
    pub driver_image: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDeviceGroupRequest {
    pub name: Option<String>,
    pub driver_image: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceGroupResponse {
    pub id: String,
    pub tenant_id: String,
    pub org_id: String,
    pub site_id: String,
    pub name: String,
    pub driver_image: String,
    pub description: Option<String>,
    pub status: String,
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
            driver_image: model.driver_image,
            description: model.description,
            status: model.status,
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
        let active_model = crate::entity::device_group::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            org_id: Set(org_id),
            site_id: Set(site_id),
            name: Set(req.name),
            driver_image: Set(req.driver_image),
            description: Set(req.description),
            status: Set("active".to_string()),
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
        if let Some(driver_image) = req.driver_image {
            model.driver_image = Set(driver_image);
        }
        if let Some(description) = req.description {
            model.description = Set(Some(description));
        }
        if let Some(status) = req.status {
            model.status = Set(status);
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
}
