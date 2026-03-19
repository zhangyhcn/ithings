use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{UserRoleEntity, UserRoleColumn, UserRoleModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleResponse {
    pub id: String,
    pub tenant_id: String,
    pub user_id: String,
    pub role_id: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignUserRoleRequest {
    pub user_id: Uuid,
    pub role_ids: Vec<Uuid>,
}

impl From<Model> for UserRoleResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.to_string(),
            user_id: model.user_id.to_string(),
            role_id: model.role_id.to_string(),
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct UserRoleService {
    db: DatabaseConnection,
}

impl UserRoleService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn assign_roles(&self, tenant_id: Uuid, req: AssignUserRoleRequest) -> Result<Vec<UserRoleResponse>, AppError> {
        // 先删除用户现有的角色
        UserRoleEntity::delete_many()
            .filter(UserRoleColumn::TenantId.eq(tenant_id))
            .filter(UserRoleColumn::UserId.eq(req.user_id))
            .exec(&self.db)
            .await?;

        let now = chrono::Utc::now().naive_utc();
        let mut results = Vec::new();

        // 批量添加新角色
        for role_id in req.role_ids {
            let active_model = crate::entity::user_role::ActiveModel {
                id: Set(Uuid::new_v4()),
                tenant_id: Set(tenant_id),
                user_id: Set(req.user_id),
                role_id: Set(role_id),
                created_at: Set(now),
                updated_at: Set(now),
            };

            let model = active_model.insert(&self.db).await?;
            results.push(UserRoleResponse::from(model));
        }

        Ok(results)
    }

    pub async fn get_user_roles(&self, tenant_id: Uuid, user_id: Uuid) -> Result<Vec<UserRoleResponse>, AppError> {
        let models = UserRoleEntity::find()
            .filter(UserRoleColumn::TenantId.eq(tenant_id))
            .filter(UserRoleColumn::UserId.eq(user_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(UserRoleResponse::from).collect())
    }

    pub async fn remove_user_role(&self, tenant_id: Uuid, user_id: Uuid, role_id: Uuid) -> Result<(), AppError> {
        let result = UserRoleEntity::delete_many()
            .filter(UserRoleColumn::TenantId.eq(tenant_id))
            .filter(UserRoleColumn::UserId.eq(user_id))
            .filter(UserRoleColumn::RoleId.eq(role_id))
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound("User role not found".to_string()));
        }

        Ok(())
    }
}
