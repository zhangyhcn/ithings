use axum::{
    extract::{Path, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    response::Response,
    service::namespace::{CreateNamespaceRequest, NamespaceResponse, NamespaceService, UpdateNamespaceRequest},
    utils::AppError,
};

pub fn create_namespace_router(db: DatabaseConnection) -> Router<DatabaseConnection> {
    Router::new()
        .route("/:tenant_id/namespaces", get(list_namespaces_by_tenant))
        .route("/:tenant_id/namespaces", post(create_namespace))
        .route("/:tenant_id/namespaces/:id", get(get_namespace))
        .route("/:tenant_id/namespaces/:id", put(update_namespace))
        .route("/:tenant_id/namespaces/:id", delete(delete_namespace))
        .with_state(db)
}

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct TenantNamespacePath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

pub async fn create_namespace(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateNamespaceRequest>,
) -> Result<Json<Response<NamespaceResponse>>, AppError> {
    let service = NamespaceService::new(db);
    let namespace = service.create(
        tenant_id,
        CreateNamespaceRequest {
            site_id: req.site_id,
            name: req.name,
            slug: req.slug,
            description: req.description,
            namespace_type: req.namespace_type,
        }
    ).await?;
    Ok(Json(Response::success(namespace)))
}

pub async fn get_namespace(
    State(db): State<DatabaseConnection>,
    Path(TenantNamespacePath { tenant_id: _, id }): Path<TenantNamespacePath>,
) -> Result<Json<Response<NamespaceResponse>>, AppError> {
    let service = NamespaceService::new(db);
    let namespace = service.find_by_id(id).await?;
    Ok(Json(Response::success(namespace)))
}

pub async fn list_namespaces_by_tenant(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<Json<Response<Vec<NamespaceResponse>>>, AppError> {
    let service = NamespaceService::new(db);
    let namespaces = service.list_by_tenant(tenant_id).await?;
    Ok(Json(Response::success(namespaces)))
}

#[derive(Debug, Deserialize)]
pub struct TenantSlugPath {
    pub tenant_slug: String,
}

pub async fn list_namespaces_by_tenant_slug(
    State(db): State<DatabaseConnection>,
    Path(TenantSlugPath { tenant_slug }): Path<TenantSlugPath>,
) -> Result<Json<Response<Vec<NamespaceResponse>>>, AppError> {
    let service = NamespaceService::new(db);
    let namespaces = service.list_by_tenant_slug(&tenant_slug).await?;
    Ok(Json(Response::success(namespaces)))
}

pub async fn update_namespace(
    State(db): State<DatabaseConnection>,
    Path(TenantNamespacePath { tenant_id: _, id }): Path<TenantNamespacePath>,
    Json(req): Json<UpdateNamespaceRequest>,
) -> Result<Json<Response<NamespaceResponse>>, AppError> {
    let service = NamespaceService::new(db);
    let namespace = service.update(id, req).await?;
    Ok(Json(Response::success(namespace)))
}

pub async fn delete_namespace(
    State(db): State<DatabaseConnection>,
    Path(TenantNamespacePath { tenant_id: _, id }): Path<TenantNamespacePath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = NamespaceService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
