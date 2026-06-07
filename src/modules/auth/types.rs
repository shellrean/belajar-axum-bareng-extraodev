use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, FromRow)]
pub struct LoginCheck {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
