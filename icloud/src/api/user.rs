use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use serde::Deserialize;
use uuid::Uuid;
use std::sync::Arc;

use crate::{
    middleware::AuthState,
    middleware::jwt_service::JwtService,
    response::{PageResponse, Response},
    service::user::{LoginRequest, LoginResponse, RegisterRequest, UserResponse, UserService, UpdateUserRequest, CreateUserRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct RefreshTokenQuery {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub tenant_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

pub fn create_user_router(db: sea_orm::DatabaseConnection, auth_state: AuthState) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/refresh", post(refresh_token))
        .route("/users/me", get(get_current_user))
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", put(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state((db, auth_state))
}

async fn register(
    State((db, auth_state)): State<(sea_orm::DatabaseConnection, AuthState)>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<Response<UserResponse>>, AppError> {
    let service = UserService::new(db, (*auth_state.jwt_service).clone());
    let user = service.register(req).await?;
    Ok(Json(Response::success(user)))
}

async fn login(
    State((db, auth_state)): State<(sea_orm::DatabaseConnection, AuthState)>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<Response<LoginResponse>>, AppError> {
    let service = UserService::new(db, (*auth_state.jwt_service).clone());
    let response = service.login(req).await?;
    Ok(Json(Response::success(response)))
}

async fn refresh_token(
    State((db, auth_state)): State<(sea_orm::DatabaseConnection, AuthState)>,
    Query(query): Query<RefreshTokenQuery>,
) -> Result<Json<Response<LoginResponse>>, AppError> {
    let service = UserService::new(db, (*auth_state.jwt_service).clone());
    let response = service.refresh_token(&query.refresh_token).await?;
    Ok(Json(Response::success(response)))
}

async fn get_current_user(
    State((db, auth_state)): State<(sea_orm::DatabaseConnection, AuthState)>,
    axum::extract::Extension(auth_user): axum::extract::Extension<crate::middleware::AuthUser>,
) -> Result<Json<Response<UserResponse>>, AppError> {
    let service = UserService::new(db, (*auth_state.jwt_service).clone());
    let user = service.get_user(auth_user.user_id).await?;
    Ok(Json(Response::success(user)))
}

async fn list_users(
    State((db, auth_state)): State<(sea_orm::DatabaseConnection, AuthState)>,
    Query(query): Query<PageQuery>,
) -> Result<Json<Response<PageResponse<UserResponse>>>, AppError> {
    let service = UserService::new(db, (*auth_state.jwt_service).clone());
    let page = query.page.unwrap_or(1) as i64;
    let page_size = query.page_size.unwrap_or(10) as i64;
    
    let (users, total) = if let Some(tenant_id) = query.tenant_id {
        service.list_by_tenant(tenant_id, page, page_size).await?
    } else {
        service.list(page, page_size).await?
    };
    
    let response = PageResponse::new(users, total, page, page_size);
    Ok(Json(Response::success(response)))
}

async fn get_user(
    State((db, auth_state)): State<(sea_orm::DatabaseConnection, AuthState)>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<UserResponse>>, AppError> {
    let service = UserService::new(db, (*auth_state.jwt_service).clone());
    let user = service.get_user(id).await?;
    Ok(Json(Response::success(user)))
}

async fn create_user(
    State((db, auth_state)): State<(sea_orm::DatabaseConnection, AuthState)>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<Response<UserResponse>>, AppError> {
    let service = UserService::new(db, (*auth_state.jwt_service).clone());
    let user = service.create(req).await?;
    Ok(Json(Response::success(user)))
}

async fn update_user(
    State((db, auth_state)): State<(sea_orm::DatabaseConnection, AuthState)>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<Json<Response<UserResponse>>, AppError> {
    let service = UserService::new(db, (*auth_state.jwt_service).clone());
    let user = service.update(id, req).await?;
    Ok(Json(Response::success(user)))
}

async fn delete_user(
    State((db, auth_state)): State<(sea_orm::DatabaseConnection, AuthState)>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<()>>, AppError> {
    let service = UserService::new(db, (*auth_state.jwt_service).clone());
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}