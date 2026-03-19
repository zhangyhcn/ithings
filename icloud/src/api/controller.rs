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
    service::controller::{ControllerService, CreateControllerRequest, ControllerResponse, UpdateControllerRequest},
    utils::AppError,
};

use axum::middleware;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub fn create_controller_router(db: DatabaseConnection, auth_state: AuthState) -> Router {
    Router::new()
        .route("/controllers", post(create_controller))
        .route("/controllers", get(list_controllers))
        .route("/controllers/:id", get(get_controller))
        .route("/controllers/:id", put(update_controller))
        .route("/controllers/:id", delete(delete_controller))
        .route("/controllers/:id/publish", post(publish_controller))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            crate::middleware::auth::auth_middleware,
        ))
        .with_state(db)
}

pub async fn create_controller(
    State(db): State<DatabaseConnection>,
    Json(req): Json<CreateControllerRequest>,
) -> Result<Json<Response<ControllerResponse>>, AppError> {
    let service = ControllerService::new(db);
    let result = service.create(req).await?;
    Ok(Json(Response::success(result)))
}

pub async fn get_controller(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<ControllerResponse>>, AppError> {
    let service = ControllerService::new(db);
    let controller = service.find_by_id(id).await?;
    Ok(Json(Response::success(controller)))
}

pub async fn list_controllers(
    State(db): State<DatabaseConnection>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Response<PageResponse<ControllerResponse>>>, AppError> {
    let service = ControllerService::new(db);
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let (list, total) = service.list(page, page_size).await?;
    let response = PageResponse::new(list, total, page, page_size);
    Ok(Json(Response::success(response)))
}

pub async fn update_controller(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateControllerRequest>,
) -> Result<Json<Response<ControllerResponse>>, AppError> {
    let service = ControllerService::new(db);
    let controller = service.update(id, req).await?;
    Ok(Json(Response::success(controller)))
}

pub async fn delete_controller(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<()>>, AppError> {
    let service = ControllerService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}

pub async fn publish_controller(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<ControllerResponse>>, AppError> {
    let service = ControllerService::new(db);
    let controller = service.publish(id).await?;
    Ok(Json(Response::success(controller)))
}
