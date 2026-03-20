use axum::{
    extract::{Path, State, Query},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    response::Response,
    service::product::{CreateProductRequest, ProductResponse, ProductService, UpdateProductRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ProductPath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

pub fn create_product_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/products", post(create_product))
        .route("/products", get(list_products))
        .route("/products/:id", get(get_product))
        .route("/products/:id", put(update_product))
        .route("/products/:id", delete(delete_product))
        .with_state(db)
}

async fn create_product(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateProductRequest>,
) -> Result<Json<Response<ProductResponse>>, AppError> {
    let service = ProductService::new(db);
    let product = service.create(tenant_id, req).await?;
    Ok(Json(Response::success(product)))
}

async fn get_product(
    State(db): State<DatabaseConnection>,
    Path(ProductPath { tenant_id: _, id }): Path<ProductPath>,
) -> Result<Json<Response<ProductResponse>>, AppError> {
    let service = ProductService::new(db);
    let product = service.find_by_id(id).await?;
    Ok(Json(Response::success(product)))
}

async fn list_products(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Query(_query): Query<PageQuery>,
) -> Result<Json<Response<Vec<ProductResponse>>>, AppError> {
    let service = ProductService::new(db);
    let products = service.list_by_tenant(tenant_id).await?;
    Ok(Json(Response::success(products)))
}

async fn update_product(
    State(db): State<DatabaseConnection>,
    Path(ProductPath { tenant_id: _, id }): Path<ProductPath>,
    Json(req): Json<UpdateProductRequest>,
) -> Result<Json<Response<ProductResponse>>, AppError> {
    let service = ProductService::new(db);
    let product = service.update(id, req).await?;
    Ok(Json(Response::success(product)))
}

async fn delete_product(
    State(db): State<DatabaseConnection>,
    Path(ProductPath { tenant_id: _, id }): Path<ProductPath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = ProductService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
