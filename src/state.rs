use sqlx::PgPool;

use crate::modules::auth::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub storage_path: String,
    pub auth_service: AuthService,
}
