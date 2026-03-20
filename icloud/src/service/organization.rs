use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    entity::organization,
    utils::AppError,
};

fn parse_optional_uuid<'de, D>(deserializer: D) -> Result<Option<Uuid>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => Uuid::parse_str(&s)
            .map(Some)
            .map_err(serde::de::Error::custom),
        None => Ok(None),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationBody {
    pub name: String,
    #[serde(deserialize_with = "parse_optional_uuid", default)]
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrganizationRequest {
    pub tenant_id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

impl From<(Uuid, CreateOrganizationBody)> for CreateOrganizationRequest {
    fn from((tenant_id, body): (Uuid, CreateOrganizationBody)) -> Self {
        Self {
            tenant_id,
            name: body.name,
            parent_id: body.parent_id,
            description: body.description,
            sort_order: body.sort_order,
            status: body.status,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    #[serde(deserialize_with = "parse_optional_uuid", default)]
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct OrganizationResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<OrganizationResponse>,
}

impl From<organization::Model> for OrganizationResponse {
    fn from(model: organization::Model) -> Self {
        Self {
            id: model.id,
            tenant_id: model.tenant_id,
            parent_id: model.parent_id,
            name: model.name,
            slug: model.slug,
            description: model.description,
            status: model.status,
            created_at: model.created_at,
            updated_at: model.updated_at,
            children: Vec::new(),
        }
    }
}

pub struct OrganizationService {
    db: DatabaseConnection,
}

impl OrganizationService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, req: CreateOrganizationRequest) -> Result<OrganizationResponse, AppError> {
        let slug = req.name.to_lowercase().replace(" ", "-");

        let org = organization::ActiveModel {
            tenant_id: Set(req.tenant_id),
            parent_id: Set(req.parent_id),
            name: Set(req.name),
            slug: Set(slug),
            description: Set(req.description),
            status: Set(req.status.unwrap_or("active".to_string())),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        };

        let org = org.insert(&self.db).await?;
        Ok(OrganizationResponse::from(org))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<OrganizationResponse, AppError> {
        let org = organization::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;
        Ok(OrganizationResponse::from(org))
    }

    pub async fn list(&self) -> Result<Vec<OrganizationResponse>, AppError> {
        let orgs = organization::Entity::find()
            .all(&self.db)
            .await?;
        
        Ok(self.build_tree(orgs))
    }

    pub async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<OrganizationResponse>, AppError> {
        let orgs = organization::Entity::find()
            .filter(organization::Column::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;
        
        Ok(self.build_tree(orgs))
    }

    fn build_tree(&self, orgs: Vec<organization::Model>) -> Vec<OrganizationResponse> {
        let mut org_responses: Vec<OrganizationResponse> = orgs.into_iter().map(OrganizationResponse::from).collect();
        
        let mut tree = Vec::new();
        let mut children_map = std::collections::HashMap::new();
        
        for org in org_responses.drain(..) {
            children_map.entry(org.parent_id).or_insert_with(Vec::new).push(org);
        }
        
        let mut stack = children_map.remove(&None).unwrap_or_default();
        
        while let Some(mut org) = stack.pop() {
            if let Some(children) = children_map.remove(&Some(org.id)) {
                org.children = children;
            }
            tree.push(org);
        }
        
        tree.reverse();
        tree
    }

    pub async fn update(&self, id: Uuid, req: UpdateOrganizationRequest) -> Result<OrganizationResponse, AppError> {
        let org = organization::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        let mut org: organization::ActiveModel = org.into();

        if let Some(name) = req.name {
            org.name = Set(name.clone());
            org.slug = Set(name.to_lowercase().replace(" ", "-"));
        }
        if let Some(parent_id) = req.parent_id {
            org.parent_id = Set(Some(parent_id));
        }
        if let Some(description) = req.description {
            org.description = Set(Some(description));
        }
        if let Some(status) = req.status {
            org.status = Set(status);
        }
        org.updated_at = Set(Utc::now().naive_utc());

        let org = org.update(&self.db).await?;
        Ok(OrganizationResponse::from(org))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let mut all_ids = vec![id];
        let mut current_level = vec![id];
        
        while !current_level.is_empty() {
            let mut next_level = Vec::new();
            for parent_id in current_level {
                let children = organization::Entity::find()
                    .filter(organization::Column::ParentId.eq(parent_id))
                    .all(&self.db)
                    .await?;
                
                for child in children {
                    all_ids.push(child.id);
                    next_level.push(child.id);
                }
            }
            current_level = next_level;
        }

        all_ids.reverse();
        for org_id in all_ids {
            let result = organization::Entity::delete_by_id(org_id)
                .exec(&self.db)
                .await?;

            if result.rows_affected == 0 {
                return Err(AppError::NotFound("Organization not found".to_string()));
            }
        }

        Ok(())
    }
}
