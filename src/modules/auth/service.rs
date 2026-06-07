use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use sqlx::PgPool;

use crate::{
    error::AppError,
    models::auth::Claim,
    modules::auth::types::{LoginRequest, LoginResponse},
};

use super::repositories::AuthRepository;

#[derive(Clone)]
pub struct AuthService {
    repo: AuthRepository,
}

impl AuthService {
    pub fn new(db: PgPool) -> Self {
        Self {
            repo: AuthRepository::new(db),
        }
    }

    pub async fn login(&self, payload: LoginRequest) -> Result<LoginResponse, AppError> {
        let row = self.repo.check_username(&payload.username).await?;
        let user = match row {
            None => return Err(AppError::InvalidLogin()),
            Some(user) => user,
        };

        let verified =
            tokio::task::spawn_blocking(move || bcrypt::verify(payload.password, &user.password))
                .await
                .map_err(|_| AppError::InvalidLogin())?
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
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| AppError::InvalidLogin())?;

        let jwt = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_ref()),
        )
        .map_err(|_| AppError::InvalidLogin())?;

        Ok(LoginResponse { token: jwt })
    }
}
