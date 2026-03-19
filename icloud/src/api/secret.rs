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
    service::secret::{SecretService, CreateSecretRequest, SecretResponse, UpdateSecretRequest},
    utils::AppError,
};

use axum::middleware;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub fn create_secret_router(db: DatabaseConnection, auth_state: AuthState) -> Router {
    Router::new()
        .route("/secrets", post(create_secret))
        .route("/secrets", get(list_secrets))
        .route("/secrets/:id", get(get_secret))
        .route("/secrets/:id", put(update_secret))
        .route("/secrets/:id", delete(delete_secret))
        .route("/secrets/:id/publish", post(publish_secret))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            crate::middleware::auth::auth_middleware,
        ))
        .with_state(db)
}

pub async fn create_secret(
    State(db): State<DatabaseConnection>,
    Json(req): Json<CreateSecretRequest>,
) -> Result<Json<Response<SecretResponse>>, AppError> {
    let service = SecretService::new(db);
    let result = service.create(req).await?;
    Ok(Json(Response::success(result)))
}

pub async fn get_secret(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<SecretResponse>>, AppError> {
    let service = SecretService::new(db);
    let secret = service.find_by_id(id).await?;
    Ok(Json(Response::success(secret)))
}

pub async fn list_secrets(
    State(db): State<DatabaseConnection>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Response<PageResponse<SecretResponse>>>, AppError> {
    let service = SecretService::new(db);
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let (list, total) = service.list(page, page_size).await?;
    let response = PageResponse::new(list, total, page, page_size);
    Ok(Json(Response::success(response)))
}

pub async fn update_secret(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSecretRequest>,
) -> Result<Json<Response<SecretResponse>>, AppError> {
    let service = SecretService::new(db);
    let secret = service.update(id, req).await?;
    Ok(Json(Response::success(secret)))
}

pub async fn delete_secret(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<()>>, AppError> {
    let service = SecretService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}

pub async fn publish_secret(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<SecretResponse>>, AppError> {
    let service = SecretService::new(db);
    let secret = service.publish(id).await?;
    Ok(Json(Response::success(secret)))
}
