// src/error.rs
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub struct AppError {
    pub status: StatusCode,
    pub message: &'static str,
}

impl AppError {
    pub fn new(status: StatusCode, message: &'static str) -> Self {
        Self { status, message }
    }

    pub fn unauthorized(msg: &'static str) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, msg)
    }

    pub fn forbidden(msg: &'static str) -> Self {
        Self::new(StatusCode::FORBIDDEN, msg)
    }

    pub fn bad_request(msg: &'static str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, msg)
    }
    pub fn too_large() -> Self {
        Self::new(
            StatusCode::PAYLOAD_TOO_LARGE,
            StatusCode::PAYLOAD_TOO_LARGE.canonical_reason().unwrap(),
        )
    }

    pub fn not_found(msg: &'static str) -> Self {
        Self::new(StatusCode::NOT_FOUND, msg)
    }

    pub fn internal(msg: &'static str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, msg)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message
        }));
        (self.status, body).into_response()
    }
}

// Удобные типы
pub type Result<T> = std::result::Result<T, AppError>;

impl ApiError {
    pub(crate) fn new(status: StatusCode, message: &'static str) -> Self {
        Self { status, message }
    }
}

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: &'static str,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({
            "error": self.message
        }));
        (self.status, body).into_response()
    }
}
