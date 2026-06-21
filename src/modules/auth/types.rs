use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use validify::Validify;

#[derive(Clone, FromRow)]
pub struct LoginCheck {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Validify)]
pub struct LoginRequest {
    #[validate(length(min = 5, max = 100, message = "Username must be 5 - 100"))]
    pub username: String,

    #[validate(length(min = 10, max = 100))]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
