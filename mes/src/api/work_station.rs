use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
    Router,
    routing::{get, post, put, delete},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::entity::work_station;
use crate::service::work_station::WorkStationService;

#[derive(Deserialize)]
pub struct CreateWorkStationRequest {
    pub station_no: String,
    pub station_name: String,
    pub workshop_id: Option<Uuid>,
    pub production_line_id: Option<Uuid>,
    pub equipment_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateWorkStationRequest {
    pub station_name: Option<String>,
    pub workshop_id: Option<Uuid>,
    pub production_line_id: Option<Uuid>,
    pub equipment_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct WorkStationResponse {
    pub id: Uuid,
    pub station_no: String,
    pub station_name: String,
    pub workshop_id: Option<Uuid>,
    pub production_line_id: Option<Uuid>,
    pub equipment_id: Option<Uuid>,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<work_station::Model> for WorkStationResponse {
    fn from(model: work_station::Model) -> Self {
        Self {
            id: model.id,
            station_no: model.station_no,
            station_name: model.station_name,
            workshop_id: model.workshop_id,
            production_line_id: model.production_line_id,
            equipment_id: model.equipment_id,
            status: model.status,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

pub fn create_work_station_router(db: Arc<sea_orm::DatabaseConnection>) -> Router {
    Router::new()
        .route("/work-stations", post(create_work_station))
        .route("/work-stations", get(list_work_stations))
        .route("/work-stations/:id", get(get_work_station))
        .route("/work-stations/:id", put(update_work_station))
        .route("/work-stations/:id", delete(delete_work_station))
        .route("/workshops/:workshop_id/work-stations", get(list_by_workshop))
        .route("/production-lines/:production_line_id/work-stations", get(list_by_production_line))
        .with_state(db)
}

async fn create_work_station(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<CreateWorkStationRequest>,
) -> Result<Json<ApiResponse<WorkStationResponse>>, StatusCode> {
    let service = WorkStationService::new(db);

    let station = service
        .create(
            tenant_id,
            req.station_no,
            req.station_name,
            req.workshop_id,
            req.production_line_id,
            req.equipment_id,
        )
        .await
        .map_err(|e| {
            eprintln!("Error creating work station: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(station.into()),
        message: "工站创建成功".to_string(),
    }))
}

async fn get_work_station(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<WorkStationResponse>>, StatusCode> {
    let service = WorkStationService::new(db);

    let station = service
        .get_by_id(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(station.into()),
        message: "查询成功".to_string(),
    }))
}

async fn list_work_stations(
    Path(tenant_id): Path<Uuid>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<WorkStationResponse>>>, StatusCode> {
    let service = WorkStationService::new(db);

    let stations = service
        .list_all(tenant_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(stations.into_iter().map(|s| s.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn update_work_station(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
    Json(req): Json<UpdateWorkStationRequest>,
) -> Result<Json<ApiResponse<WorkStationResponse>>, StatusCode> {
    let service = WorkStationService::new(db);

    let station = service
        .update(
            tenant_id,
            id,
            req.station_name,
            req.workshop_id,
            req.production_line_id,
            req.equipment_id,
            req.status,
        )
        .await
        .map_err(|e| {
            eprintln!("Error updating work station: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(station.into()),
        message: "工站更新成功".to_string(),
    }))
}

async fn delete_work_station(
    Path((tenant_id, id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<()>>, StatusCode> {
    let service = WorkStationService::new(db);

    service
        .delete(tenant_id, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: None,
        message: "工站删除成功".to_string(),
    }))
}

async fn list_by_workshop(
    Path((tenant_id, workshop_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<WorkStationResponse>>>, StatusCode> {
    let service = WorkStationService::new(db);

    let stations = service
        .list_by_workshop(tenant_id, workshop_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(stations.into_iter().map(|s| s.into()).collect()),
        message: "查询成功".to_string(),
    }))
}

async fn list_by_production_line(
    Path((tenant_id, production_line_id)): Path<(Uuid, Uuid)>,
    State(db): State<Arc<sea_orm::DatabaseConnection>>,
) -> Result<Json<ApiResponse<Vec<WorkStationResponse>>>, StatusCode> {
    let service = WorkStationService::new(db);

    let stations = service
        .list_by_production_line(tenant_id, production_line_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiResponse {
        success: true,
        data: Some(stations.into_iter().map(|s| s.into()).collect()),
        message: "查询成功".to_string(),
    }))
}
