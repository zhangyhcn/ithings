use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppError {
    pub message: String,
    pub code: u16,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

impl From<sea_orm::error::DbErr> for AppError {
    fn from(err: sea_orm::error::DbErr) -> Self {
        AppError {
            message: err.to_string(),
            code: 500,
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError {
            message: err.to_string(),
            code: 400,
        }
    }
}

impl AppError {
    pub fn not_found(msg: String) -> Self {
        AppError {
            message: msg,
            code: 404,
        }
    }

    pub fn bad_request(msg: String) -> Self {
        AppError {
            message: msg,
            code: 400,
        }
    }

    pub fn internal_server_error(msg: String) -> Self {
        AppError {
            message: msg,
            code: 500,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let body = Json(serde_json::json!({
            "code": self.code,
            "message": self.message,
        }));
        (status, body).into_response()
    }
}
