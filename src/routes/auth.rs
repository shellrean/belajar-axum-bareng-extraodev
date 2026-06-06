use std::env;
use axum::extract::State;
use axum::Json;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use tower::ServiceExt;
use crate::error::AppError;
use crate::models::auth::{Claim, LoginRequest, LoginResponse};
use crate::state::AppState;

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, AppError> {
    let row = sqlx::query!("SELECT id, username, password FROM users WHERE username = $1",
        payload.username)
        .fetch_optional(&app_state.db)
        .await?;
    let user = row.ok_or_else(||AppError::InvalidLogin())?;

    let verified = tokio::task::spawn_blocking(move || {
        bcrypt::verify(payload.password, &user.password)
    }).await
        .map_err(|_| {
            AppError::InvalidLogin()
        })?
        .map_err(|_| AppError::InvalidLogin())?;

    if !verified {
        return Err(AppError::InvalidLogin());
    }

    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(10))
        .ok_or_else(|| AppError::InvalidLogin())?;

    let claims = Claim {
        sub: user.username,
        exp: expiration.timestamp() as usize,
    };
    let jwt_secret = env::var("JWT_SECRET")
        .map_err(|_| AppError::InvalidLogin())?;
    
    let jwt = jsonwebtoken::encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(jwt_secret.as_ref())
    ).map_err(|_| AppError::InvalidLogin())?;

    Ok(Json(LoginResponse {
        token: jwt
    }))
}