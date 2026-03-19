use axum::{
    extract::{Extension, Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    middleware::{AuthState, AuthUser},
    response::Response,
    service::role::{CreateRoleRequest, RoleResponse, RoleService, UpdateRoleRequest},
    utils::AppError,
};

pub fn create_role_router(db: DatabaseConnection, auth_state: AuthState) -> Router {
    Router::new()
        .route("/roles", post(create_role))
        .route("/roles", get(list_roles))
        .route("/roles/{id}", get(get_role))
        .route("/roles/{id}", put(update_role))
        .route("/roles/{id}", delete(delete_role))
        .with_state(db)
        .layer(axum::middleware::from_fn_with_state(
            auth_state,
            crate::middleware::auth::auth_middleware,
        ))
}

async fn create_role(
    State(db): State<DatabaseConnection>,
    Extension(auth_user): Extension<AuthUser>,
    Json(mut req): Json<CreateRoleRequest>,
) -> Result<Json<Response<RoleResponse>>, AppError> {
    // 使用当前用户的租户ID
    req.tenant_id = auth_user.tenant_id.ok_or_else(|| AppError::BadRequest("User has no tenant".to_string()))?;
    
    let service = RoleService::new(db);
    let role = service.create(req).await?;
    Ok(Json(Response::success(role)))
}

async fn get_role(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<RoleResponse>>, AppError> {
    let service = RoleService::new(db);
    let role = service.find_by_id(id).await?;
    Ok(Json(Response::success(role)))
}

async fn list_roles(
    State(db): State<DatabaseConnection>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Response<Vec<RoleResponse>>>, AppError> {
    let tenant_id = auth_user.tenant_id.ok_or_else(|| AppError::BadRequest("User has no tenant".to_string()))?;
    
    let service = RoleService::new(db);
    let roles = service.list_by_tenant(tenant_id).await?;
    Ok(Json(Response::success(roles)))
}

async fn update_role(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<Json<Response<RoleResponse>>, AppError> {
    let service = RoleService::new(db);
    let role = service.update(id, req).await?;
    Ok(Json(Response::success(role)))
}

async fn delete_role(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<()>>, AppError> {
    let service = RoleService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
