use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, put},
    Router,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    response::Response,
    service::node::{NodeResponse, NodeService, UpdateLabelsRequest},
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

pub fn create_node_router(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/nodes", get(list_nodes))
        .route("/nodes/sync", get(sync_nodes))
        .route("/nodes/:id", get(get_node))
        .route("/nodes/:id/labels", put(update_node_labels))
        .with_state(db)
}

async fn list_nodes(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<Json<Response<Vec<NodeResponse>>>, AppError> {
    let service = NodeService::new(db);
    let nodes = service.list_all().await?;
    Ok(Json(Response::success(nodes)))
}

async fn sync_nodes(
    State(db): State<DatabaseConnection>,
    Path(TenantPath { tenant_id }): Path<TenantPath>,
) -> Result<Json<Response<Vec<NodeResponse>>>, AppError> {
    let service = NodeService::new(db);
    let nodes = service.sync_from_k8s().await?;
    Ok(Json(Response::success(nodes)))
}

async fn get_node(
    State(db): State<DatabaseConnection>,
    Path(NodePath { tenant_id, id }): Path<NodePath>,
) -> Result<Json<Response<NodeResponse>>, AppError> {
    let service = NodeService::new(db);
    let node = service.find_by_id(id).await?;
    Ok(Json(Response::success(node)))
}

async fn update_node_labels(
    State(db): State<DatabaseConnection>,
    Path(NodePath { tenant_id, id }): Path<NodePath>,
    Json(req): Json<UpdateLabelsRequest>,
) -> Result<Json<Response<NodeResponse>>, AppError> {
    let service = NodeService::new(db);
    let node = service.update_labels(id, req).await?;
    Ok(Json(Response::success(node)))
}
