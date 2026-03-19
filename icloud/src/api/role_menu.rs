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
    service::role_menu::{AssignRoleMenusRequest, RoleMenuResponse, RoleMenuService},
    utils::AppError,
};

pub fn create_role_menu_router(db: DatabaseConnection, auth_state: crate::middleware::AuthState) -> Router {
    Router::new()
        .route("/role_menus/:role_id", get(get_role_menus))
        .route("/role_menus", post(assign_role_menus))
        .route("/role_menus/:role_id/:menu_id", delete(remove_role_menu))
        .with_state(db)
        .layer(axum::middleware::from_fn_with_state(
            auth_state,
            crate::middleware::auth::auth_middleware,
        ))
}

async fn get_role_menus(
    State(db): State<DatabaseConnection>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(role_id): Path<Uuid>,
) -> Result<Json<Response<Vec<RoleMenuResponse>>>, AppError> {
    let service = RoleMenuService::new(db);
    let role_menus = service.get_role_menus(role_id).await?;
    Ok(Json(Response::success(role_menus)))
}

async fn assign_role_menus(
    State(db): State<DatabaseConnection>,
    Extension(_auth_user): Extension<AuthUser>,
    Json(req): Json<AssignRoleMenusRequest>,
) -> Result<Json<Response<Vec<RoleMenuResponse>>>, AppError> {
    let service = RoleMenuService::new(db);
    let role_menus = service.assign_menus(req).await?;
    Ok(Json(Response::success(role_menus)))
}

async fn remove_role_menu(
    State(db): State<DatabaseConnection>,
    Extension(_auth_user): Extension<AuthUser>,
    Path((role_id, menu_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Response<()>>, AppError> {
    let service = RoleMenuService::new(db);
    service.remove_role_menu(role_id, menu_id).await?;
    Ok(Json(Response::success(())))
}
