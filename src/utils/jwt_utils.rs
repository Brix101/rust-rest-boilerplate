use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

use crate::config::AppConfig;

use super::errors::{AppError, AppResult};
// use mockall::automock;

/// A security service for handling JWT authentication.
pub type DynJwtUtil = Arc<dyn JwtUtil + Send + Sync>;

// #[automock]
pub trait JwtUtil {
    fn new_access_token(&self, user_id: i64, email: &str) -> AppResult<String>;
    fn new_refresh_token(&self, sub: i64) -> AppResult<String>;
    fn get_user_id_from_token(&self, token: String) -> AppResult<i64>;
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct AccessTokenClaims {
    sub: String,
    user_id: i64,
    exp: usize,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct RefreshTokenClaims {
    sub: i64,
    exp: usize,
}

pub struct JwtTokenUtil {
    config: Arc<AppConfig>,
}

impl JwtTokenUtil {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl JwtUtil for JwtTokenUtil {
    fn new_access_token(&self, user_id: i64, email: &str) -> AppResult<String> {
        let from_now = Duration::from_secs(86400);
        let expired_future_time = SystemTime::now().add(from_now);
        let exp = OffsetDateTime::from(expired_future_time);

        let claims = AccessTokenClaims {
            sub: String::from(email),
            exp: exp.unix_timestamp() as usize,
            user_id,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.access_token_secret.as_bytes()),
        )
        .map_err(|err| AppError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(token)
    }

    fn new_refresh_token(&self, sub: i64) -> AppResult<String> {
        let from_now = Duration::from_secs(86400);
        let expired_future_time = SystemTime::now().add(from_now);
        let exp = OffsetDateTime::from(expired_future_time);

        let claims = RefreshTokenClaims {
            sub,
            exp: exp.unix_timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.access_token_secret.as_bytes()),
        )
        .map_err(|err| AppError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(token)
    }

    fn get_user_id_from_token(&self, token: String) -> AppResult<i64> {
        let decoded_token = decode::<AccessTokenClaims>(
            token.as_str(),
            &DecodingKey::from_secret(self.config.access_token_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|err| AppError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(decoded_token.claims.user_id)
    }
}
