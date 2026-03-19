use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordVerifier};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, PaginatorTrait, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::entity::{UserEntity, UserColumn, UserModel as Model};
use crate::entity::user::ActiveModel as UserActiveModel;
use crate::middleware::JwtService;
use crate::utils::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub is_active: Option<bool>,
    pub tenant_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub tenant_id: Option<String>,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub role: String,
    pub is_superuser: bool,
    pub is_active: bool,
    pub last_login: Option<String>,
    pub created_at: String,
}

impl From<Model> for UserResponse {
    fn from(model: Model) -> Self {
        Self {
            id: model.id.to_string(),
            tenant_id: model.tenant_id.map(|t| t.to_string()),
            username: model.username,
            email: model.email,
            phone: model.phone,
            role: model.role,
            is_superuser: model.is_superuser,
            is_active: model.is_active,
            last_login: model.last_login.map(|t| t.to_string()),
            created_at: model.created_at.to_string(),
        }
    }
}

pub struct UserService {
    db: DatabaseConnection,
    jwt_service: JwtService,
}

impl UserService {
    pub fn new(db: DatabaseConnection, jwt_service: JwtService) -> Self {
        Self { db, jwt_service }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<UserResponse, AppError> {
        let existing_email = UserEntity::find()
            .filter(UserColumn::Email.eq(&req.email))
            .one(&self.db)
            .await?;

        if existing_email.is_some() {
            return Err(AppError::UserAlreadyExists);
        }

        let password_hash = self.hash_password(&req.password)?;
        let now = chrono::Utc::now().naive_utc();

        let active_model = UserActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(None),
            username: Set(req.username),
            email: Set(req.email),
            password_hash: Set(password_hash),
            phone: Set(req.phone),
            role: Set("user".to_string()),
            is_superuser: Set(false),
            is_active: Set(true),
            last_login: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AppError> {
        let model = UserEntity::find()
            .filter(UserColumn::Username.eq(&req.username))
            .one(&self.db)
            .await?
            .ok_or(AppError::InvalidCredentials)?;

        if !model.is_active {
            return Err(AppError::Unauthorized("User is inactive".to_string()));
        }

        if !self.verify_password(&req.password, &model.password_hash)? {
            return Err(AppError::InvalidCredentials);
        }

        let now = chrono::Utc::now().naive_utc();
        let mut active_model = model.clone().into_active_model();
        active_model.last_login = Set(Some(now));
        active_model.update(&self.db).await?;

        let access_token = self.jwt_service.generate_token(
            model.id,
            model.tenant_id,
            model.username.clone(),
            model.role.clone(),
            model.is_superuser,
        )?;

        let refresh_token = self.jwt_service.generate_refresh_token(
            model.id,
            model.tenant_id,
            model.username.clone(),
            model.role.clone(),
            model.is_superuser,
        )?;

        Ok(LoginResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: UserResponse::from(model),
        })
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<LoginResponse, AppError> {
        let claims = self.jwt_service.verify_token(refresh_token)?;

        let user = UserEntity::find_by_id(claims.user_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::UserNotFound)?;

        if !user.is_active {
            return Err(AppError::Unauthorized("User is inactive".to_string()));
        }

        let access_token = self.jwt_service.generate_token(
            user.id,
            user.tenant_id,
            user.username.clone(),
            user.role.clone(),
            user.is_superuser,
        )?;

        let new_refresh_token = self.jwt_service.generate_refresh_token(
            user.id,
            user.tenant_id,
            user.username.clone(),
            user.role.clone(),
            user.is_superuser,
        )?;

        Ok(LoginResponse {
            access_token,
            refresh_token: new_refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            user: user.into(),
        })
    }

    pub async fn get_user(&self, user_id: Uuid) -> Result<UserResponse, AppError> {
        let model = UserEntity::find_by_id(user_id)
            .one(&self.db)
            .await?
            .ok_or(AppError::UserNotFound)?;

        Ok(model.into())
    }

    pub async fn list(&self, page: i64, page_size: i64) -> Result<(Vec<UserResponse>, i64), AppError> {
        let paginator = UserEntity::find()
            .order_by_asc(UserColumn::CreatedAt)
            .paginate(&self.db, page_size as u64);
        
        let total = paginator.num_items().await? as i64;
        let models: Vec<Model> = paginator.fetch_page(page as u64 - 1).await?;

        Ok((
            models.into_iter().map(|m: Model| m.into()).collect(),
            total
        ))
    }

    pub async fn list_by_tenant(&self, tenant_id: Uuid, page: i64, page_size: i64) -> Result<(Vec<UserResponse>, i64), AppError> {
        let paginator = UserEntity::find()
            .filter(UserColumn::TenantId.eq(Some(tenant_id)))
            .order_by_asc(UserColumn::CreatedAt)
            .paginate(&self.db, page_size as u64);
        
        let total = paginator.num_items().await? as i64;
        let models: Vec<Model> = paginator.fetch_page(page as u64 - 1).await?;

        Ok((
            models.into_iter().map(|m: Model| m.into()).collect(),
            total
        ))
    }

    pub async fn create(&self, req: CreateUserRequest) -> Result<UserResponse, AppError> {
        let existing_email = UserEntity::find()
            .filter(UserColumn::Email.eq(&req.email))
            .one(&self.db)
            .await?;

        if existing_email.is_some() {
            return Err(AppError::UserAlreadyExists);
        }

        let password_hash = self.hash_password(&req.password)?;
        let now = chrono::Utc::now().naive_utc();

        let active_model = UserActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(req.tenant_id),
            username: Set(req.username),
            email: Set(req.email),
            password_hash: Set(password_hash),
            phone: Set(None),
            role: Set(req.role),
            is_superuser: Set(false),
            is_active: Set(req.is_active.unwrap_or(true)),
            last_login: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let model = active_model.insert(&self.db).await?;
        Ok(model.into())
    }

    pub async fn update(&self, id: Uuid, req: UpdateUserRequest) -> Result<UserResponse, AppError> {
        let model = UserEntity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::UserNotFound)?;

        let mut active_model = model.into_active_model();
        let now = chrono::Utc::now().naive_utc();

        if let Some(username) = req.username {
            active_model.username = Set(username);
        }
        if let Some(email) = req.email {
            active_model.email = Set(email);
        }
        if let Some(password) = req.password {
            let password_hash = self.hash_password(&password)?;
            active_model.password_hash = Set(password_hash);
        }
        if let Some(role) = req.role {
            active_model.role = Set(role);
        }
        if let Some(is_active) = req.is_active {
            active_model.is_active = Set(is_active);
        }
        active_model.updated_at = Set(now);

        let model = active_model.update(&self.db).await?;
        Ok(model.into())
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let result = UserEntity::delete_by_id(id).exec(&self.db).await?;
        if result.rows_affected == 0 {
            return Err(AppError::UserNotFound);
        }
        Ok(())
    }

    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = Argon2::default();
        Ok(argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalServerError(format!("Failed to hash password: {}", e)))?
            .to_string())
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = argon2::PasswordHash::new(hash)
            .map_err(|e| AppError::InternalServerError(format!("Invalid password hash: {}", e)))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}