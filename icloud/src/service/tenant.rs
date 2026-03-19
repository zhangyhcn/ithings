use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, PaginatorTrait, Set, prelude::Json};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::entity::{TenantEntity, TenantColumn, TenantModel as Model};
use crate::entity::{UserEntity, UserColumn};
use crate::entity::user::ActiveModel as UserActiveModel;
use crate::entity::{role, user_role, role_menu, menu};
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTenantRequest {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub admin_username: String,
    pub admin_email: String,
    pub admin_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTenantResponse {
    pub tenant: TenantResponse,
    pub admin_user: crate::service::user::UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub config: Option<JsonValue>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TenantResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub config: JsonValue,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Model> for TenantResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            name: model.name,
            slug: model.slug,
            description: model.description,
            config: model.config.unwrap_or_else(|| JsonValue::Object(serde_json::Map::new())),
            status: model.status,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }
}

pub struct TenantService {
    db: DatabaseConnection,
}

impl TenantService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(&self, req: CreateTenantRequest) -> Result<CreateTenantResponse, AppError> {
        let existing = TenantEntity::find()
            .filter(TenantColumn::Slug.eq(&req.slug))
            .one(&self.db)
            .await?;

        if existing.is_some() {
            return Err(AppError::TenantAlreadyExists);
        }

        let existing_email = UserEntity::find()
            .filter(UserColumn::Email.eq(&req.admin_email))
            .one(&self.db)
            .await?;

        if existing_email.is_some() {
            return Err(AppError::UserAlreadyExists);
        }

        let now = chrono::Utc::now().naive_utc();
        let active_model = crate::entity::tenant::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(req.name),
            slug: Set(req.slug),
            description: Set(req.description),
            config: Set(Some(JsonValue::Object(serde_json::Map::new()))),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let tenant = active_model.insert(&self.db).await?;

        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(req.admin_password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))?
            .to_string();

        let user_active_model = UserActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(Some(tenant.id)),
            username: Set(req.admin_username),
            email: Set(req.admin_email),
            password_hash: Set(password_hash),
            phone: Set(None),
            role: Set("admin".to_string()),
            is_superuser: Set(false),
            is_active: Set(true),
            last_login: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        // 为新租户创建管理员角色
        let admin_role_id = Uuid::new_v4();
        let admin_role = role::ActiveModel {
            id: Set(admin_role_id),
            tenant_id: Set(tenant.id),
            name: Set("管理员".to_string()),
            slug: Set("admin".to_string()),
            description: Set(Some("租户管理员，拥有所有权限".to_string())),
            permissions: Set(Json::Array(vec![])),
            status: Set("active".to_string()),
            created_at: Set(now),
            updated_at: Set(now),
        };
        let admin_role = admin_role.insert(&self.db).await?;

        // 为该租户的admin角色分配所有菜单权限
        let all_menus = menu::Entity::find().all(&self.db).await?;
        for menu in all_menus {
            let role_menu_active_model = role_menu::ActiveModel {
                role_id: Set(admin_role.id),
                menu_id: Set(menu.id),
                ..Default::default()
            };
            role_menu_active_model.insert(&self.db).await?;
        }

        let admin_user = user_active_model.insert(&self.db).await?;

        // 为管理员用户分配admin角色
        let user_role_active_model = user_role::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant.id),
            user_id: Set(admin_user.id),
            role_id: Set(admin_role.id),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        user_role_active_model.insert(&self.db).await?;

        Ok(CreateTenantResponse {
            tenant: tenant.into(),
            admin_user: admin_user.into(),
        })
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<TenantResponse, AppError> {
        let model = TenantEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::TenantNotFound)?;

        Ok(model.into())
    }

    pub async fn find_by_slug(&self, slug: &str) -> Result<TenantResponse, AppError> {
        let model = TenantEntity::find()
            .filter(TenantColumn::Slug.eq(slug))
            .one(&self.db)
            .await?
            .ok_or(AppError::TenantNotFound)?;

        Ok(model.into())
    }

    pub async fn list(
        &self,
        page: i64,
        page_size: i64,
    ) -> Result<(Vec<TenantResponse>, i64), AppError> {
        let page = if page < 1 { 1 } else { page };
        let page_size = if page_size < 1 { 10 } else { page_size };

        let paginator = TenantEntity::find().paginate(&self.db, page_size as u64);
        let total = paginator.num_items().await? as i64;
        let models: Vec<Model> = paginator.fetch().await?;

        Ok((models.into_iter().map(|m| m.into()).collect(), total))
    }

    pub async fn update(
        &self,
        id: Uuid,
        req: UpdateTenantRequest,
    ) -> Result<TenantResponse, AppError> {
        let mut model = TenantEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::TenantNotFound)?
            .into_active_model();

        if let Some(name) = req.name {
            model.name = Set(name);
        }
        if let Some(description) = req.description {
            model.description = Set(Some(description));
        }
        if let Some(config) = req.config {
            model.config = Set(Some(config));
        }
        if let Some(status) = req.status {
            model.status = Set(status);
        }
        model.updated_at = Set(chrono::Utc::now().naive_utc());

        let model = model.update(&self.db).await?;
        Ok(model.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = TenantEntity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::TenantNotFound);
        }
        Ok(())
    }
}
