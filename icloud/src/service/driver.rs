use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, PaginatorTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{DriverEntity, DriverColumn, DriverModel as Model};
use crate::entity::tenant::{self};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateDriverRequest {
    pub name: String,
    pub description: Option<String>,
    pub protocol_type: String,
    pub image: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateDriverRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub protocol_type: Option<String>,
    pub image: Option<String>,
    pub version: Option<String>,
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
    pub created_at: String,
    pub updated_at: String,
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

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateDriverRequest,
    ) -> Result<DriverResponse, AppError> {
        let tenant = tenant::Entity::find()
            .filter(tenant::Column::Id.eq(tenant_id))
            .one(&self.db)
            .await?;

        if tenant.is_none() {
            return Err(AppError::TenantNotFound);
        }

        let mut active_model = crate::entity::driver::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            name: Set(req.name),
            description: Set(req.description),
            protocol_type: Set(req.protocol_type),
            image: Set(req.image),
            version: Set(req.version),
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
        let models = DriverEntity::find()
            .filter(DriverColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
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
}
