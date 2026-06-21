use crate::error::AppError;
use crate::models::auth::{Claim, UserContext};
use axum::extract::Request;
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::env;

tokio::task_local! {
    pub static USER_CONTEXT: UserContext;
}

pub async fn validate_token(req: Request, next: Next) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::InvalidToken())?;

    let jwt_key = env::var("JWT_SECRET").map_err(|_| AppError::InvalidToken())?;

    let claim = decode::<Claim>(
        token,
        &DecodingKey::from_secret(jwt_key.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| AppError::InvalidToken())?;

    let user_context = UserContext {
        username: claim.claims.sub,
    };

    Ok(USER_CONTEXT.scope(user_context, next.run(req)).await)
}
