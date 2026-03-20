use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, PaginatorTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::entity::{ProductEntity, ProductColumn, ProductModel as Model};
use crate::entity::tenant::{self};
use crate::utils::AppError;
use device_common::device_core::{ThingModel, Rule};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub thing_model: Option<JsonValue>,
    #[serde(default)]
    pub rule: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub thing_model: Option<JsonValue>,
    pub rule: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub description: Option<String>,
    pub thing_model: JsonValue,
    pub rule: JsonValue,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for ProductResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            name: model.name,
            description: model.description,
            thing_model: model.thing_model,
            rule: model.rule,
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

    fn validate_thing_model(thing_model: &JsonValue) -> Result<(), AppError> {
        if thing_model.is_null() || thing_model.as_object().map_or(true, |obj| obj.is_empty()) {
            return Ok(());
        }

        let model: ThingModel = serde_json::from_value(thing_model.clone())
            .map_err(|e| AppError::Validation(format!("物模型格式错误: {}", e)))?;

        model.validate()
            .map_err(|e| AppError::Validation(format!("物模型校验失败: {}", e)))?;

        Ok(())
    }

    fn validate_rule(rule: &JsonValue) -> Result<(), AppError> {
        if rule.is_null() || rule.as_object().map_or(true, |obj| obj.is_empty()) {
            return Ok(());
        }

        if let Some(rules_array) = rule.as_array() {
            for rule_value in rules_array {
                let _: Rule = serde_json::from_value(rule_value.clone())
                    .map_err(|e| AppError::Validation(format!("规则格式错误: {}", e)))?;
            }
        } else {
            let _: Rule = serde_json::from_value(rule.clone())
                .map_err(|e| AppError::Validation(format!("规则格式错误: {}", e)))?;
        }

        Ok(())
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let tenant = tenant::Entity::find()
            .filter(tenant::Column::Id.eq(tenant_id))
            .one(&self.db)
            .await?;

        if tenant.is_none() {
            return Err(AppError::TenantNotFound);
        }

        if let Some(ref thing_model) = req.thing_model {
            Self::validate_thing_model(thing_model)?;
        }

        if let Some(ref rule) = req.rule {
            Self::validate_rule(rule)?;
        }

        let mut active_model = crate::entity::product::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            name: Set(req.name),
            description: Set(req.description),
            thing_model: Set(req.thing_model.unwrap_or(serde_json::json!({}))),
            rule: Set(req.rule.unwrap_or(serde_json::json!({}))),
            ..Default::default()
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<ProductResponse, AppError> {
        let model = ProductEntity::find()
            .filter(ProductColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::NotFound("Product not found".to_string())),
        }
    }

    pub async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ProductResponse>, AppError> {
        let models = ProductEntity::find()
            .filter(ProductColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
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

        let Some(mut model) = model else {
            return Err(AppError::NotFound("Product not found".to_string()));
        };

        if let Some(ref thing_model) = req.thing_model {
            Self::validate_thing_model(thing_model)?;
        }

        if let Some(ref rule) = req.rule {
            Self::validate_rule(rule)?;
        }

        let mut active_model = model.into_active_model();

        if let Some(name) = req.name {
            active_model.name = Set(name);
        }
        if let Some(description) = req.description {
            active_model.description = Set(Some(description));
        }
        if let Some(thing_model) = req.thing_model {
            active_model.thing_model = Set(thing_model);
        }
        if let Some(rule) = req.rule {
            active_model.rule = Set(rule);
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
            return Err(AppError::NotFound("Product not found".to_string()));
        };

        ProductEntity::delete(model.into_active_model()).exec(&self.db).await?;
        Ok(())
    }
}
