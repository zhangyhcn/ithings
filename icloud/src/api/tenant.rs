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
    service::tenant::{CreateTenantRequest, CreateTenantResponse, TenantResponse, TenantService, UpdateTenantRequest},
    utils::AppError,
};
use crate::api::namespace;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

pub fn create_tenant_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/tenants", post(create_tenant))
        .route("/tenants", get(list_tenants))
        .route("/tenants/:id", get(get_tenant))
        .route("/tenants/:id", put(update_tenant))
        .route("/tenants/:id", delete(delete_tenant))
        .nest("/tenants", namespace::create_namespace_router(db.clone()))
        .with_state(db)
}

pub async fn create_tenant(
    State(db): State<DatabaseConnection>,
    Json(req): Json<CreateTenantRequest>,
) -> Result<Json<Response<CreateTenantResponse>>, AppError> {
    let service = TenantService::new(db);
    let result = service.create(req).await?;
    Ok(Json(Response::success(result)))
}

pub async fn get_tenant(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<TenantResponse>>, AppError> {
    let service = TenantService::new(db);
    let tenant = service.find_by_id(id).await?;
    Ok(Json(Response::success(tenant)))
}

pub async fn list_tenants(
    State(db): State<DatabaseConnection>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Response<PageResponse<TenantResponse>>>, AppError> {
    let service = TenantService::new(db);
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(10);

    let (list, total) = service.list(page, page_size).await?;
    let response = PageResponse::new(list, total, page, page_size);
    Ok(Json(Response::success(response)))
}

pub async fn update_tenant(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTenantRequest>,
) -> Result<Json<Response<TenantResponse>>, AppError> {
    let service = TenantService::new(db);
    let tenant = service.update(id, req).await?;
    Ok(Json(Response::success(tenant)))
}

pub async fn delete_tenant(
    State(db): State<DatabaseConnection>,
    Path(id): Path<Uuid>,
) -> Result<Json<Response<()>>, AppError> {
    let service = TenantService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
