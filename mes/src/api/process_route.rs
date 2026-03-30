use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    response::Response,
    service::process_route::{ProcessRouteService, CreateProcessRouteRequest, UpdateProcessRouteRequest, ProcessRouteResponse},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ProcessRoutePath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ProductPath {
    pub tenant_id: Uuid,
    pub product_id: Uuid,
}

pub fn create_process_route_router(db: Arc<DatabaseConnection>) -> Router {
    Router::new()
        .route("/process-routes", get(list_process_routes).post(create_process_route))
        .route("/process-routes/:id", get(get_process_route).put(update_process_route).delete(delete_process_route))
        .route("/process-routes/:id/set-default", post(set_default_process_route))
        .route("/products/:product_id/process-routes", get(list_process_routes_by_product))
        .with_state(db)
}

async fn list_process_routes(
    State(db): State<Arc<DatabaseConnection>>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<ResponseJson<Response<Vec<ProcessRouteResponse>>>, AppError> {
    let service = ProcessRouteService::new(db);
    let routes = service.list_all(tenant_id).await?;
    Ok(ResponseJson(Response::success(routes)))
}

async fn list_process_routes_by_product(
    State(db): State<Arc<DatabaseConnection>>,
    Path(ProductPath { tenant_id, product_id }): Path<ProductPath>,
) -> Result<ResponseJson<Response<Vec<ProcessRouteResponse>>>, AppError> {
    let service = ProcessRouteService::new(db);
    let routes = service.list_by_product(tenant_id, product_id).await?;
    Ok(ResponseJson(Response::success(routes)))
}

async fn create_process_route(
    State(db): State<Arc<DatabaseConnection>>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateProcessRouteRequest>,
) -> Result<ResponseJson<Response<ProcessRouteResponse>>, AppError> {
    let service = ProcessRouteService::new(db);
    let route = service.create(tenant_id, req).await?;
    Ok(ResponseJson(Response::success(route)))
}

async fn get_process_route(
    State(db): State<Arc<DatabaseConnection>>,
    Path(ProcessRoutePath { tenant_id: _, id }): Path<ProcessRoutePath>,
) -> Result<ResponseJson<Response<ProcessRouteResponse>>, AppError> {
    let service = ProcessRouteService::new(db);
    let route = service.find_by_id(id).await?;
    Ok(ResponseJson(Response::success(route)))
}

async fn update_process_route(
    State(db): State<Arc<DatabaseConnection>>,
    Path(ProcessRoutePath { tenant_id: _, id }): Path<ProcessRoutePath>,
    Json(req): Json<UpdateProcessRouteRequest>,
) -> Result<ResponseJson<Response<ProcessRouteResponse>>, AppError> {
    let service = ProcessRouteService::new(db);
    let route = service.update(id, req).await?;
    Ok(ResponseJson(Response::success(route)))
}

async fn delete_process_route(
    State(db): State<Arc<DatabaseConnection>>,
    Path(ProcessRoutePath { tenant_id: _, id }): Path<ProcessRoutePath>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = ProcessRouteService::new(db);
    service.delete(id).await?;
    Ok(ResponseJson(Response::success(())))
}

async fn set_default_process_route(
    State(db): State<Arc<DatabaseConnection>>,
    Path(ProcessRoutePath { tenant_id: _, id }): Path<ProcessRoutePath>,
) -> Result<ResponseJson<Response<ProcessRouteResponse>>, AppError> {
    let service = ProcessRouteService::new(db);
    let route = service.set_as_default(id).await?;
    Ok(ResponseJson(Response::success(route)))
}
