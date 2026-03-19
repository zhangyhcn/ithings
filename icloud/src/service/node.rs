use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, PaginatorTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{NodeEntity, NodeColumn, NodeModel as Model};
use crate::entity::tenant::{self};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNodeRequest {
    pub name: String,
    pub address: Option<String>,
    pub k8s_context: Option<String>,
    pub is_shared: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNodeRequest {
    pub name: Option<String>,
    pub address: Option<String>,
    pub k8s_context: Option<String>,
    pub is_shared: Option<bool>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeResponse {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub address: Option<String>,
    pub k8s_context: Option<String>,
    pub is_shared: bool,
    pub status: String,
    pub last_sync: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for NodeResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            name: model.name,
            address: model.address,
            k8s_context: model.k8s_context,
            is_shared: model.is_shared,
            status: model.status.clone(),
            last_sync: model.last_sync.map(|dt| dt.to_string()),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct NodeService {
    db: DatabaseConnection,
}

impl NodeService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        tenant_id: Uuid,
        req: CreateNodeRequest,
    ) -> Result<NodeResponse, AppError> {
        let tenant = tenant::Entity::find()
            .filter(tenant::Column::Id.eq(tenant_id))
            .one(&self.db)
            .await?;

        if tenant.is_none() {
            return Err(AppError::TenantNotFound);
        }

        let mut active_model = crate::entity::node::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            name: Set(req.name),
            address: Set(req.address),
            k8s_context: Set(req.k8s_context),
            is_shared: Set(req.is_shared),
            status: Set("online".to_string()),
            ..Default::default()
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<NodeResponse, AppError> {
        let model = NodeEntity::find()
            .filter(NodeColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        match model {
            Some(model) => Ok(model.into()),
            None => Err(AppError::NotFound("Node not found".to_string())),
        }
    }

    pub async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<NodeResponse>, AppError> {
        let models = NodeEntity::find()
            .filter(NodeColumn::TenantId.eq(tenant_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(Into::into).collect())
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateNodeRequest,
    ) -> Result<NodeResponse, AppError> {
        let model = NodeEntity::find()
            .filter(NodeColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(mut model) = model else {
            return Err(AppError::NotFound("Node not found".to_string()));
        };

        let mut active_model = model.into_active_model();

        if let Some(name) = req.name {
            active_model.name = Set(name);
        }
        if let Some(address) = req.address {
            active_model.address = Set(Some(address));
        }
        if let Some(k8s_context) = req.k8s_context {
            active_model.k8s_context = Set(Some(k8s_context));
        }
        if let Some(is_shared) = req.is_shared {
            active_model.is_shared = Set(is_shared);
        }
        if let Some(status) = req.status {
            active_model.status = Set(status);
        }

        let updated = active_model.update(&self.db).await?;
        Ok(updated.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let model = NodeEntity::find()
            .filter(NodeColumn::Id.eq(id))
            .one(&self.db)
            .await?;

        let Some(model) = model else {
            return Err(AppError::NotFound("Node not found".to_string()));
        };

        NodeEntity::delete(model.into_active_model()).exec(&self.db).await?;
        Ok(())
    }
}
