use axum::extract::multipart::MultipartError;
use axum::http::StatusCode;
use axum::Json;
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
}

#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    message: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, data) = match &self {
            AppError::DatabaseError(e) => {
                eprintln!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    status: 99,
                    message: "A database error occurred".to_string()
                })
            },
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND,
                ErrorResponse {
                    status: 40,
                    message: format!("Resource not found: {}", msg)
                })
            },
            AppError::InvalidLogin() => {
                (StatusCode::UNAUTHORIZED, ErrorResponse {
                    status: 41,
                    message: "Invalid username or password".into()
                })
            },
            AppError::InvalidToken() => {
                (StatusCode::UNAUTHORIZED, ErrorResponse {
                    status: 42,
                    message: "Invalid or missing token".into()
                })
            },
            AppError::ErrorMultipart(e) => {
                (StatusCode::BAD_REQUEST, ErrorResponse {
                    status: 40,
                    message: e.to_string()
                })
            },
            AppError::IoError(e) => {
                (StatusCode::BAD_REQUEST, ErrorResponse {
                    status: 40,
                    message: e.to_string()
                })
            }
        };
        let body = Json(data);
        (status, body).into_response()
    }
}