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
    service::organization::{CreateOrganizationRequest, OrganizationResponse, OrganizationService, UpdateOrganizationRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct OrganizationPath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

pub fn create_organization_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/organizations", post(create_organization))
        .route("/organizations", get(list_organizations))
        .route("/organizations/:id", get(get_organization))
        .route("/organizations/:id", put(update_organization))
        .route("/organizations/:id", delete(delete_organization))
        .with_state(db)
}

async fn create_organization(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateOrganizationRequest>,
) -> Result<Json<Response<OrganizationResponse>>, AppError> {
    let service = OrganizationService::new(db);
    let org = service.create(CreateOrganizationRequest {
        tenant_id,
        name: req.name,
        parent_id: req.parent_id,
        description: req.description,
        sort_order: req.sort_order,
        status: req.status,
    }).await?;
    Ok(Json(Response::success(org)))
}

async fn get_organization(
    State(db): State<DatabaseConnection>,
    Path(OrganizationPath { tenant_id: _, id }): Path<OrganizationPath>,
) -> Result<Json<Response<OrganizationResponse>>, AppError> {
    let service = OrganizationService::new(db);
    let org = service.find_by_id(id).await?;
    Ok(Json(Response::success(org)))
}

async fn list_organizations(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<Json<Response<Vec<OrganizationResponse>>>, AppError> {
    let service = OrganizationService::new(db);
    let orgs = service.list_by_tenant(tenant_id).await?;
    Ok(Json(Response::success(orgs)))
}

async fn update_organization(
    State(db): State<DatabaseConnection>,
    Path(OrganizationPath { tenant_id: _, id }): Path<OrganizationPath>,
    Json(req): Json<UpdateOrganizationRequest>,
) -> Result<Json<Response<OrganizationResponse>>, AppError> {
    let service = OrganizationService::new(db);
    let org = service.update(id, req).await?;
    Ok(Json(Response::success(org)))
}

async fn delete_organization(
    State(db): State<DatabaseConnection>,
    Path(OrganizationPath { tenant_id: _, id }): Path<OrganizationPath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = OrganizationService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}