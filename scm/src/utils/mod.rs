use axum::{
    http::StatusCode,
    response::{IntoResponse, Response as HttpResponse},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    BadRequest(String),
    
    #[error("{0}")]
    NotFound(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("{0}")]
    DatabaseError(#[from] sea_orm::DbErr),
}

impl AppError {
    pub fn bad_request(msg: String) -> Self {
        AppError::BadRequest(msg)
    }
    
    pub fn not_found(msg: String) -> Self {
        AppError::NotFound(msg)
    }
    
    pub fn internal(msg: String) -> Self {
        AppError::Internal(msg)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> HttpResponse {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
        };
        
        let body = Json(json!({
            "code": status.as_u16(),
            "message": message,
            "data": Option::<()>::None,
        }));
        
        (status, body).into_response()
    }
}
