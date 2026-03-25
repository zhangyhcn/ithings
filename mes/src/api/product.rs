use axum::{
    extract::{Path, State, Json},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    response::Response,
    service::product::{ProductService, CreateProductRequest, UpdateProductRequest, ProductResponse},
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

pub fn create_product_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/products", get(list_products).post(create_product))
        .route("/products/:id", get(get_product).put(update_product).delete(delete_product))
        .with_state(db)
}

async fn list_products(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<ResponseJson<Response<Vec<ProductResponse>>>, AppError> {
    let service = ProductService::new(db);
    let products = service.list_all(tenant_id).await?;
    Ok(ResponseJson(Response::success(products)))
}

async fn create_product(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateProductRequest>,
) -> Result<ResponseJson<Response<ProductResponse>>, AppError> {
    let service = ProductService::new(db);
    let product = service.create(tenant_id, req).await?;
    Ok(ResponseJson(Response::success(product)))
}

async fn get_product(
    State(db): State<DatabaseConnection>,
    Path(ProductPath { tenant_id: _, id }): Path<ProductPath>,
) -> Result<ResponseJson<Response<ProductResponse>>, AppError> {
    let service = ProductService::new(db);
    let product = service.find_by_id(id).await?;
    Ok(ResponseJson(Response::success(product)))
}

async fn update_product(
    State(db): State<DatabaseConnection>,
    Path(ProductPath { tenant_id: _, id }): Path<ProductPath>,
    Json(req): Json<UpdateProductRequest>,
) -> Result<ResponseJson<Response<ProductResponse>>, AppError> {
    let service = ProductService::new(db);
    let product = service.update(id, req).await?;
    Ok(ResponseJson(Response::success(product)))
}

async fn delete_product(
    State(db): State<DatabaseConnection>,
    Path(ProductPath { tenant_id: _, id }): Path<ProductPath>,
) -> Result<ResponseJson<Response<()>>, AppError> {
    let service = ProductService::new(db);
    service.delete(id).await?;
    Ok(ResponseJson(Response::success(())))
}
