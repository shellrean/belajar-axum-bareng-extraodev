use std::collections::HashMap;

use axum::Json;
use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Invalid login")]
    InvalidLogin(),

    #[error("Invalid token")]
    InvalidToken(),

    #[error("Error multipart")]
    ErrorMultipart(#[from] MultipartError),

    #[error("Io Error")]
    IoError(#[from] std::io::Error),

    #[error("Validation error: {0}")]
    ValidationError(#[from] validify::ValidationErrors),
}

#[derive(Serialize, Default)]
struct ErrorResponse {
    status: u16,
    message: String,
    errors: HashMap<String, Vec<String>>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, data) = match &self {
            AppError::DatabaseError(e) => {
                eprintln!("Database error: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        status: 99,
                        message: "A database error occurred".to_string(),
                        ..Default::default()
                    },
                )
            }
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: 40,
                    message: format!("Resource not found: {}", msg),
                    ..Default::default()
                },
            ),
            AppError::InvalidLogin() => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    status: 41,
                    message: "Invalid username or password".into(),
                    ..Default::default()
                },
            ),
            AppError::InvalidToken() => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    status: 42,
                    message: "Invalid or missing token".into(),
                    ..Default::default()
                },
            ),
            AppError::ErrorMultipart(e) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    status: 40,
                    message: e.to_string(),
                    ..Default::default()
                },
            ),
            AppError::IoError(e) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    status: 40,
                    message: e.to_string(),
                    ..Default::default()
                },
            ),
            AppError::ValidationError(e) => {
                let mut errors: HashMap<String, Vec<String>> = HashMap::new();
                for error in e.field_errors() {
                    if let (Some(field_name), Some(message)) = (error.field_name(), error.message())
                    {
                        errors
                            .entry(field_name.to_string())
                            .or_default()
                            .push(message.to_string());
                    }
                }

                (
                    StatusCode::BAD_REQUEST,
                    ErrorResponse {
                        status: 40,
                        message: String::from("validation error"),
                        errors: errors,
                    },
                )
            }
        };
        let body = Json(data);
        (status, body).into_response()
    }
}
