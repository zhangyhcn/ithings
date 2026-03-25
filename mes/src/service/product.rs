use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::entity::{ProductEntity, ProductColumn, ProductModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub product_no: String,
    pub name: String,
    pub specification: Option<String>,
    pub unit: Option<String>,
    pub product_type: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub specification: Option<String>,
    pub unit: Option<String>,
    pub product_type: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: String,
    pub tenant_id: String,
    pub product_no: String,
    pub name: String,
    pub specification: Option<String>,
    pub unit: Option<String>,
    pub product_type: Option<String>,
    pub description: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for ProductResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            product_no: model.product_no,
            name: model.name,
            specification: model.specification,
            unit: model.unit,
            product_type: model.product_type,
            description: model.description,
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct ProductService {
    db: DatabaseConnection,
}

impl ProductService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let now = Utc::now().naive_utc();
        let active_model = crate::entity::product::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            product_no: Set(req.product_no),
            name: Set(req.name),
            specification: Set(req.specification),
            unit: Set(req.unit),
            product_type: Set(req.product_type),
            description: Set(req.description),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn list_all(&self, tenant_id: Uuid) -> Result<Vec<ProductResponse>, AppError> {
        let models = ProductEntity::find()
            .filter(ProductColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<ProductResponse, AppError> {
        let model = ProductEntity::find()
            .filter(ProductColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::not_found("Product not found".to_string())),
        }
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let model = ProductEntity::find()
            .filter(ProductColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(model) = model else {
            return Err(AppError::not_found("Product not found".to_string()));
        };

        let mut active_model = crate::entity::product::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            product_no: Set(model.product_no),
            name: Set(model.name),
            specification: Set(model.specification),
            unit: Set(model.unit),
            product_type: Set(model.product_type),
            description: Set(model.description),
            status: Set(model.status),
            created_at: Set(model.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };

        if let Some(name) = req.name {
            active_model.name = Set(name);
        }
        if let Some(specification) = req.specification {
            active_model.specification = Set(Some(specification));
        }
        if let Some(unit) = req.unit {
            active_model.unit = Set(Some(unit));
        }
        if let Some(product_type) = req.product_type {
            active_model.product_type = Set(Some(product_type));
        }
        if let Some(description) = req.description {
            active_model.description = Set(Some(description));
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }

        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let model = ProductEntity::find()
            .filter(ProductColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(model) = model else {
            return Err(AppError::not_found("Product not found".to_string()));
        };

        let mut active_model = crate::entity::product::ActiveModel {
            id: Set(model.id),
            tenant_id: Set(model.tenant_id),
            product_no: Set(model.product_no),
            name: Set(model.name),
            specification: Set(model.specification),
            unit: Set(model.unit),
            product_type: Set(model.product_type),
            description: Set(model.description),
            status: Set("deleted".to_string()),
            created_at: Set(model.created_at),
            updated_at: Set(Utc::now().naive_utc()),
        };

        active_model.update(&self.db).await?;
        Ok(())
    }
}
