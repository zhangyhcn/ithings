use axum::{
    extract::{Extension, Path, State},
    response::Json,
    routing::{delete, get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    middleware::AuthUser,
    response::Response,
    service::user_role::{AssignUserRoleRequest, UserRoleResponse, UserRoleService},
    utils::AppError,
};

pub fn create_user_role_router(db: DatabaseConnection, auth_state: crate::middleware::AuthState) -> Router {
    Router::new()
        .route("/user_roles/:user_id", get(get_user_roles))
        .route("/user_roles", post(assign_user_roles))
        .route("/user_roles/:user_id/:role_id", delete(remove_user_role))
        .with_state(db)
        .layer(axum::middleware::from_fn_with_state(
            auth_state,
            crate::middleware::auth::auth_middleware,
        ))
}

async fn get_user_roles(
    State(db): State<DatabaseConnection>,
    Extension(auth_user): Extension<AuthUser>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<Response<Vec<UserRoleResponse>>>, AppError> {
    let tenant_id = auth_user.tenant_id.ok_or_else(|| AppError::BadRequest("User has no tenant".to_string()))?;
    
    let service = UserRoleService::new(db);
    let user_roles = service.get_user_roles(tenant_id, user_id).await?;
    Ok(Json(Response::success(user_roles)))
}

async fn assign_user_roles(
    State(db): State<DatabaseConnection>,
    Extension(auth_user): Extension<AuthUser>,
    Json(req): Json<AssignUserRoleRequest>,
) -> Result<Json<Response<Vec<UserRoleResponse>>>, AppError> {
    let tenant_id = auth_user.tenant_id.ok_or_else(|| AppError::BadRequest("User has no tenant".to_string()))?;
    
    let service = UserRoleService::new(db);
    let user_roles = service.assign_roles(tenant_id, req).await?;
    Ok(Json(Response::success(user_roles)))
}

async fn remove_user_role(
    State(db): State<DatabaseConnection>,
    Extension(auth_user): Extension<AuthUser>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Response<()>>, AppError> {
    let tenant_id = auth_user.tenant_id.ok_or_else(|| AppError::BadRequest("User has no tenant".to_string()))?;
    
    let service = UserRoleService::new(db);
    service.remove_user_role(tenant_id, user_id, role_id).await?;
    Ok(Json(Response::success(())))
}
