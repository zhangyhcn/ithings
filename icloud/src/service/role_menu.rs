use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{RoleMenuEntity, RoleMenuColumn, RoleMenuModel as Model};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleMenuResponse {
    pub id: i32,
    pub role_id: String,
    pub menu_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignRoleMenusRequest {
    pub role_id: Uuid,
    pub menu_ids: Vec<Uuid>,
}

impl From<Model> for RoleMenuResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            role_id: model.role_id.to_string(),
            menu_id: model.menu_id.to_string(),
        }
    }
}

pub struct RoleMenuService {
    db: DatabaseConnection,
}

impl RoleMenuService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn assign_menus(&self, req: AssignRoleMenusRequest) -> Result<Vec<RoleMenuResponse>, AppError> {
        // 先删除角色现有的菜单权限
        RoleMenuEntity::delete_many()
            .filter(RoleMenuColumn::RoleId.eq(req.role_id))
            .exec(&self.db)
            .await?;

        let mut results = Vec::new();

        // 批量添加新菜单权限
        for menu_id in req.menu_ids {
            let active_model = crate::entity::role_menu::ActiveModel {
                role_id: Set(req.role_id),
                menu_id: Set(menu_id),
                ..Default::default()
            };

            let model = active_model.insert(&self.db).await?;
            results.push(RoleMenuResponse::from(model));
        }

        Ok(results)
    }

    pub async fn get_role_menus(&self, role_id: Uuid) -> Result<Vec<RoleMenuResponse>, AppError> {
        let models = RoleMenuEntity::find()
            .filter(RoleMenuColumn::RoleId.eq(role_id))
            .all(&self.db)
            .await?;

        Ok(models.into_iter().map(RoleMenuResponse::from).collect())
    }

    pub async fn remove_role_menu(&self, role_id: Uuid, menu_id: Uuid) -> Result<(), AppError> {
        let result = RoleMenuEntity::delete_many()
            .filter(RoleMenuColumn::RoleId.eq(role_id))
            .filter(RoleMenuColumn::MenuId.eq(menu_id))
            .exec(&self.db)
            .await?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Role menu not found".to_string()));
        }

        Ok(())
    }
}
