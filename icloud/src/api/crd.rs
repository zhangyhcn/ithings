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
    service::crd::{CrdService, CreateCrdRequest, CrdResponse, UpdateCrdRequest},
    utils::AppError,
};

use axum::middleware;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub fn create_crd_router(db: DatabaseConnection, auth_state: AuthState) -> Router {
    Router::new()
        .route("/crdes", post(create_crd))
        .route("/crdes", get(list_crds))
        .route("/crdes/:id", get(get_crd))
        .route("/crdes/:id", put(update_crd))
        .route("/crdes/:id", delete(delete_crd))
        .route("/crdes/:id/publish", post(publish_crd))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            crate::middleware::auth::auth_middleware,
        ))
        .with_state(db)
}

pub async fn create_crd(
    State(db): State<DatabaseConnection>,
    Json(req): Json<CreateCrdRequest>,
) -> Result<Json<Response<CrdResponse>>, AppError> {
    let service = CrdService::new(db);
    let result = service.create(req).await?;
    Ok(Json(Response::success(result)))
}

pub async fn get_crd(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<CrdResponse>>, AppError> {
    let service = CrdService::new(db);
    let crd = service.find_by_id(id).await?;
    Ok(Json(Response::success(crd)))
}

pub async fn list_crds(
    State(db): State<DatabaseConnection>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Response<PageResponse<CrdResponse>>>, AppError> {
    let service = CrdService::new(db);
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let (list, total) = service.list(page, page_size).await?;
    let response = PageResponse::new(list, total, page, page_size);
    Ok(Json(Response::success(response)))
}

pub async fn update_crd(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCrdRequest>,
) -> Result<Json<Response<CrdResponse>>, AppError> {
    let service = CrdService::new(db);
    let crd = service.update(id, req).await?;
    Ok(Json(Response::success(crd)))
}

pub async fn delete_crd(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<()>>, AppError> {
    let service = CrdService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}

pub async fn publish_crd(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<CrdResponse>>, AppError> {
    let service = CrdService::new(db);
    let crd = service.publish(id).await?;
    Ok(Json(Response::success(crd)))
}
