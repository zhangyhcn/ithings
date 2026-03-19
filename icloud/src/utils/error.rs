use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum AppError {
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("NotFound: {0}")]
    NotFound(String),

    #[error("BadRequest: {0}")]
    BadRequest(String),

    #[error("InternalServerError: {0}")]
    InternalServerError(String),

    #[error("Tenant not found")]
    TenantNotFound,

    #[error("User not found")]
    UserNotFound,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Tenant already exists")]
    TenantAlreadyExists,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,
}

impl AppError {
    pub fn code(&self) -> i32 {
        match self {
            Self::Unauthorized(_) => 401,
            Self::Forbidden(_) => 403,
            Self::NotFound(_) => 404,
            Self::BadRequest(_) => 400,
            Self::InternalServerError(_) => 500,
            Self::TenantNotFound => 404,
            Self::UserNotFound => 404,
            Self::InvalidCredentials => 401,
            Self::UserAlreadyExists => 409,
            Self::TenantAlreadyExists => 409,
            Self::TokenExpired => 401,
            Self::InvalidToken => 401,
        }
    }
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::InternalServerError(err.to_string())
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.code() {
            401 => axum::http::StatusCode::UNAUTHORIZED,
            403 => axum::http::StatusCode::FORBIDDEN,
            404 => axum::http::StatusCode::NOT_FOUND,
            409 => axum::http::StatusCode::CONFLICT,
            400 => axum::http::StatusCode::BAD_REQUEST,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({
            "code": self.code(),
            "message": self.to_string(),
            "data": serde_json::Value::Null
        });

        (status, axum::Json(body)).into_response()
    }
}
