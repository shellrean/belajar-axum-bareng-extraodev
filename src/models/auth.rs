use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
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

#[derive(Clone)]
pub struct UserContext {
    pub username: String
}