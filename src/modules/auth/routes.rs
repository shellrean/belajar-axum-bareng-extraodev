use axum::{Json, extract::State};

use super::types::{LoginRequest, LoginResponse};
use crate::{error::AppError, state::AppState};

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let result = app_state.auth_service.login(payload).await?;

    Ok(Json(result))
}
