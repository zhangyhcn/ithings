use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::middleware::jwt::Claims;
use crate::middleware::jwt_service::JwtService;
use crate::utils::AppError;
use crate::entity::TokenBlacklistEntity;
use crate::entity::token_blacklist::Column as TokenBlacklistColumn;
use sea_orm::EntityTrait;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub username: String,
    pub role: String,
    pub is_superuser: bool,
    pub token: Option<String>,
}

impl From<Claims> for AuthUser {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.user_id,
            tenant_id: claims.tenant_id,
            username: claims.sub,
            role: claims.role,
            is_superuser: claims.is_superuser,
            token: None,
        }
    }
}

#[derive(Clone)]
pub struct AuthState {
    pub jwt_service: Arc<JwtService>,
    pub db: DatabaseConnection,
}

impl AuthState {
    pub fn new(jwt_service: JwtService, db: DatabaseConnection) -> Self {
        Self {
            jwt_service: Arc::new(jwt_service),
            db,
        }
    }
}

pub async fn auth_middleware(
    state: State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_token_from_header(request.headers())?;

    if token.is_empty() {
        return Err(AppError::Unauthorized("Missing token".to_string()));
    }

    // 检查token是否在黑名单中
    let is_blacklisted = TokenBlacklistEntity::find()
        .filter(TokenBlacklistColumn::Token.eq(&token))
        .one(&state.db)
        .await?;

    if is_blacklisted.is_some() {
        return Err(AppError::Unauthorized("Token has been revoked".to_string()));
    }

    let claims = state.jwt_service.verify_token(&token)?;
    let mut auth_user: AuthUser = claims.into();
    auth_user.token = Some(token);

    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}

fn extract_token_from_header(
    headers: &axum::http::HeaderMap,
) -> Result<String, AppError> {
    let auth_header = headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            Ok(header[7..].to_string())
        }
        Some(_) => Err(AppError::InvalidToken),
        None => Err(AppError::Unauthorized("Missing token".to_string())),
    }
}

pub async fn require_superuser(
    AuthUser { is_superuser, .. }: AuthUser,
) -> Result<AuthUser, AppError> {
    if !is_superuser {
        return Err(AppError::Forbidden("Superuser required".to_string()));
    }
    Ok(AuthUser {
        is_superuser,
        ..Default::default()
    })
}

impl Default for AuthUser {
    fn default() -> Self {
        Self {
            user_id: Uuid::nil(),
            tenant_id: None,
            username: String::new(),
            role: String::new(),
            is_superuser: false,
            token: None,
        }
    }
}
