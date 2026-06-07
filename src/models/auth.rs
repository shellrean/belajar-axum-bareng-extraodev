use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Claim {
    pub sub: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct UserContext {
    pub username: String,
}
