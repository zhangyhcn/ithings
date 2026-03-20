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
    service::node::{CreateNodeRequest, NodeResponse, NodeService, UpdateNodeRequest},
    utils::AppError,
};

#[derive(Debug, Deserialize)]
pub struct TenantPath {
    pub tenant_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct NodePath {
    pub tenant_id: Uuid,
    pub id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

pub fn create_node_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/nodes", post(create_node))
        .route("/nodes", get(list_nodes))
        .route("/nodes/:id", get(get_node))
        .route("/nodes/:id", put(update_node))
        .route("/nodes/:id", delete(delete_node))
        .with_state(db)
}

async fn create_node(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Json(req): Json<CreateNodeRequest>,
) -> Result<Json<Response<NodeResponse>>, AppError> {
    let service = NodeService::new(db);
    let node = service.create(
        tenant_id,
        CreateNodeRequest {
            name: req.name,
            address: req.address,
            k8s_context: req.k8s_context,
            is_shared: req.is_shared,
        }
    ).await?;
    Ok(Json(Response::success(node)))
}

async fn get_node(
    State(db): State<DatabaseConnection>,
    Path(NodePath { tenant_id: _, id }): Path<NodePath>,
) -> Result<Json<Response<NodeResponse>>, AppError> {
    let service = NodeService::new(db);
    let node = service.find_by_id(id).await?;
    Ok(Json(Response::success(node)))
}

async fn list_nodes(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
    Query(_query): Query<PageQuery>,
) -> Result<Json<Response<Vec<NodeResponse>>>, AppError> {
    let service = NodeService::new(db);
    let nodes = service.list_by_tenant(tenant_id).await?;
    Ok(Json(Response::success(nodes)))
}

async fn update_node(
    State(db): State<DatabaseConnection>,
    Path(NodePath { tenant_id: _, id }): Path<NodePath>,
    Json(req): Json<UpdateNodeRequest>,
) -> Result<Json<Response<NodeResponse>>, AppError> {
    let service = NodeService::new(db);
    let node = service.update(id, req).await?;
    Ok(Json(Response::success(node)))
}

async fn delete_node(
    State(db): State<DatabaseConnection>,
    Path(NodePath { tenant_id: _, id }): Path<NodePath>,
) -> Result<Json<Response<()>>, AppError> {
    let service = NodeService::new(db);
    service.delete(id).await?;
    Ok(Json(Response::success(())))
}
