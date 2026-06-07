use sqlx::PgPool;

use crate::error::AppError;

use super::types::LoginCheck;

#[derive(Clone)]
pub struct AuthRepository {
    db: PgPool,
}

impl AuthRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn check_username(&self, username: &str) -> Result<Option<LoginCheck>, AppError> {
        let user = sqlx::query_as::<_, LoginCheck>(
            "SELECT id, username, password FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.db)
        .await?;

        Ok(user)
    }
}
