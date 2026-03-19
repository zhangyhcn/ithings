use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    middleware::{AuthState, AuthUser},
    response::{PageResponse, Response},
    service::config_map::{ConfigMapService, CreateConfigMapRequest, ConfigMapResponse, UpdateConfigMapRequest},
    utils::AppError,
};

use axum::middleware;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub fn create_config_map_router(db: DatabaseConnection, auth_state: AuthState) -> Router {
    Router::new()
        .route("/config_maps", post(create_config_map))
        .route("/config_maps", get(list_config_maps))
        .route("/config_maps/:id", get(get_config_map))
        .route("/config_maps/:id", put(update_config_map))
        .route("/config_maps/:id", delete(delete_config_map))
        .route("/config_maps/:id/publish", post(publish_config_map))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            crate::middleware::auth::auth_middleware,
        ))
        .with_state(db)
}

pub async fn create_config_map(
    State(db): State<DatabaseConnection>,
    Json(req): Json<CreateConfigMapRequest>,
) -> Result<Json<Response<ConfigMapResponse>>, AppError> {
    let service = ConfigMapService::new(db);
    let result = service.create(req).await?;
    Ok(Json(Response::success(result)))
}

pub async fn get_config_map(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<ConfigMapResponse>>, AppError> {
    let service = ConfigMapService::new(db);
    let config_map = service.find_by_id(id).await?;
    Ok(Json(Response::success(config_map)))
}

pub async fn list_config_maps(
    State(db): State<DatabaseConnection>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Response<PageResponse<ConfigMapResponse>>>, AppError> {
    let service = ConfigMapService::new(db);
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let (list, total) = service.list(page, page_size).await?;
    let response = PageResponse::new(list, total, page, page_size);
    Ok(Json(Response::success(response)))
}

pub async fn update_config_map(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateConfigMapRequest>,
) -> Result<Json<Response<ConfigMapResponse>>, AppError> {
    let service = ConfigMapService::new(db);
    let config_map = service.update(id, req).await?;
    Ok(Json(Response::success(config_map)))
}

pub async fn delete_config_map(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<()>>, AppError> {
    let service = ConfigMapService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}

pub async fn publish_config_map(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<ConfigMapResponse>>, AppError> {
    let service = ConfigMapService::new(db);
    let config_map = service.publish(id).await?;
    Ok(Json(Response::success(config_map)))
}
