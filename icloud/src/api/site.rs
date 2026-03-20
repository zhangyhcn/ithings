use axum::{
    extract::{Path, State, Query},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    response::Response,
    service::site::{CreateSiteRequest, SiteResponse, SiteService, UpdateSiteRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct SitePath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

pub fn create_site_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/sites", post(create_site))
        .route("/sites", get(list_sites))
        .route("/sites/:id", get(get_site))
        .route("/sites/:id", put(update_site))
        .route("/sites/:id", delete(delete_site))
        .with_state(db)
}

async fn create_site(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateSiteRequest>,
) -> Result<Json<Response<SiteResponse>>, AppError> {
    let service = SiteService::new(db);
    let site = service.create(tenant_id, req).await?;
    Ok(Json(Response::success(site)))
}

async fn get_site(
    State(db): State<DatabaseConnection>,
    Path(SitePath { tenant_id: _, id }): Path<SitePath>,
) -> Result<Json<Response<SiteResponse>>, AppError> {
    let service = SiteService::new(db);
    let site = service.find_by_id(id).await?;
    Ok(Json(Response::success(site)))
}

async fn list_sites(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Query(_query): Query<PageQuery>,
) -> Result<Json<Response<Vec<SiteResponse>>>, AppError> {
    let service = SiteService::new(db);
    let sites = service.list_by_tenant(tenant_id).await?;
    Ok(Json(Response::success(sites)))
}

async fn update_site(
    State(db): State<DatabaseConnection>,
    Path(SitePath { tenant_id: _, id }): Path<SitePath>,
    Json(req): Json<UpdateSiteRequest>,
) -> Result<Json<Response<SiteResponse>>, AppError> {
    let service = SiteService::new(db);
    let site = service.update(id, req).await?;
    Ok(Json(Response::success(site)))
}

async fn delete_site(
    State(db): State<DatabaseConnection>,
    Path(SitePath { tenant_id: _, id }): Path<SitePath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = SiteService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
