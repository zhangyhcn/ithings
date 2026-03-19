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
    service::operator::{OperatorService, CreateOperatorRequest, OperatorResponse, UpdateOperatorRequest},
    utils::AppError,
};

use axum::middleware;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub fn create_operator_router(db: DatabaseConnection, auth_state: AuthState) -> Router {
    Router::new()
        .route("/operators", post(create_operator))
        .route("/operators", get(list_operators))
        .route("/operators/:id", get(get_operator))
        .route("/operators/:id", put(update_operator))
        .route("/operators/:id", delete(delete_operator))
        .route("/operators/:id/publish", post(publish_operator))
        .layer(middleware::from_fn_with_state(
            auth_state.clone(),
            crate::middleware::auth::auth_middleware,
        ))
        .with_state(db)
}

pub async fn create_operator(
    State(db): State<DatabaseConnection>,
    Json(req): Json<CreateOperatorRequest>,
) -> Result<Json<Response<OperatorResponse>>, AppError> {
    let service = OperatorService::new(db);
    let result = service.create(req).await?;
    Ok(Json(Response::success(result)))
}

pub async fn get_operator(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<OperatorResponse>>, AppError> {
    let service = OperatorService::new(db);
    let operator = service.find_by_id(id).await?;
    Ok(Json(Response::success(operator)))
}

pub async fn list_operators(
    State(db): State<DatabaseConnection>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Response<PageResponse<OperatorResponse>>>, AppError> {
    let service = OperatorService::new(db);
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let (list, total) = service.list(page, page_size).await?;
    let response = PageResponse::new(list, total, page, page_size);
    Ok(Json(Response::success(response)))
}

pub async fn update_operator(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateOperatorRequest>,
) -> Result<Json<Response<OperatorResponse>>, AppError> {
    let service = OperatorService::new(db);
    let operator = service.update(id, req).await?;
    Ok(Json(Response::success(operator)))
}

pub async fn delete_operator(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<()>>, AppError> {
    let service = OperatorService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}

pub async fn publish_operator(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<OperatorResponse>>, AppError> {
    let service = OperatorService::new(db);
    let operator = service.publish(id).await?;
    Ok(Json(Response::success(operator)))
}
