use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    response::Json,
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use crate::service::menu::{MenuService, MenuCreateRequest, MenuUpdateRequest, MenuTree};
use crate::middleware::AuthUser;
use crate::middleware::AuthState;
use crate::response::Response;
use crate::utils::AppError;

use axum::middleware;

pub fn create_menu_router(db: DatabaseConnection, auth_state: AuthState) -> Router {
    Router::new()
        .route("/menus", get(get_menu_tree))
        .route("/menus/user", get(get_user_menus))
        .route("/menus", post(create_menu))
        .route("/menus/:id", get(get_menu))
        .route("/menus/:id", put(update_menu))
        .route("/menus/:id", delete(delete_menu))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            crate::middleware::auth::auth_middleware,
        ))
        .with_state((db, auth_state))
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

async fn get_menu_tree(
    State((db, _auth_state)): State<(DatabaseConnection, AuthState)>,
    Extension(_auth_user): Extension<AuthUser>,
) -> Result<Json<Response<Vec<MenuTree>>>, AppError> {
    let menus = MenuService::get_menu_tree(&db).await?;
    Ok(Json(Response::success(menus)))
}

async fn get_user_menus(
    State((db, _auth_state)): State<(DatabaseConnection, AuthState)>,
    Extension(auth_user): Extension<AuthUser>,
) -> Result<Json<Response<Vec<MenuTree>>>, AppError> {
    let menus = MenuService::get_user_menus(&db, auth_user.username).await?;
    Ok(Json(Response::success(menus)))
}

async fn create_menu(
    State((db, _auth_state)): State<(DatabaseConnection, AuthState)>,
    Extension(_auth_user): Extension<AuthUser>,
    Json(req): Json<MenuCreateRequest>,
) -> Result<Json<Response<MenuTree>>, AppError> {
    let menu = MenuService::create(&db, req).await?;
    let menu_tree = MenuTree {
        id: menu.id.to_string(),
        parent_id: menu.parent_id.map(|p| p.to_string()),
        name: menu.name,
        path: menu.path,
        component: menu.component,
        icon: menu.icon,
        sort_order: menu.sort_order,
        status: menu.status,
        roles: serde_json::from_value(menu.roles).map_err(|e| AppError::InternalServerError(e.to_string()))?,
        i18n_key: menu.i18n_key,
        children: None,
    };
    Ok(Json(Response::success(menu_tree)))
}

async fn get_menu(
    State((db, _auth_state)): State<(DatabaseConnection, AuthState)>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<Json<Response<MenuTree>>, AppError> {
    let menu = MenuService::get_by_id(&db, id).await?;
    let menu_tree = MenuTree {
        id: menu.id.to_string(),
        parent_id: menu.parent_id.map(|p| p.to_string()),
        name: menu.name,
        path: menu.path,
        component: menu.component,
        icon: menu.icon,
        sort_order: menu.sort_order,
        status: menu.status,
        roles: serde_json::from_value(menu.roles).map_err(|e| AppError::InternalServerError(e.to_string()))?,
        i18n_key: menu.i18n_key,
        children: None,
    };
    Ok(Json(Response::success(menu_tree)))
}

async fn update_menu(
    State((db, _auth_state)): State<(DatabaseConnection, AuthState)>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(id): Path<String>,
    Json(req): Json<MenuUpdateRequest>,
) -> Result<Json<Response<MenuTree>>, AppError> {
    let menu = MenuService::update(&db, id, req).await?;
    let menu_tree = MenuTree {
        id: menu.id.to_string(),
        parent_id: menu.parent_id.map(|p| p.to_string()),
        name: menu.name,
        path: menu.path,
        component: menu.component,
        icon: menu.icon,
        sort_order: menu.sort_order,
        status: menu.status,
        roles: serde_json::from_value(menu.roles).map_err(|e| AppError::InternalServerError(e.to_string()))?,
        i18n_key: menu.i18n_key,
        children: None,
    };
    Ok(Json(Response::success(menu_tree)))
}

async fn delete_menu(
    State((db, _auth_state)): State<(DatabaseConnection, AuthState)>,
    Extension(_auth_user): Extension<AuthUser>,
    Path(id): Path<String>,
) -> Result<Json<Response<()>>, AppError> {
    MenuService::delete(&db, id).await?;
    Ok(Json(Response::success(())))
}
