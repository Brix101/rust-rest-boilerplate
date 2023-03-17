use std::sync::Arc;

use argon2::Config;

use crate::config::AppConfig;

use super::errors::{AppError, AppResult};
// use mockall::automock;

/// A security service for handling JWT authentication.
pub type DynArgonService = Arc<dyn ArgonService + Send + Sync>;

// #[automock]
pub trait ArgonService {
    fn hash_password(&self, raw_password: &str) -> AppResult<String>;

    fn verify_password(
        &self,
        stored_password: &str,
        attempted_password: String,
    ) -> AppResult<bool>;
}

pub struct ArgonSecurityService {
    config: Arc<AppConfig>,
}

impl ArgonSecurityService {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl ArgonService for ArgonSecurityService {
    fn hash_password(&self, raw_password: &str) -> AppResult<String> {
        let password_bytes = raw_password.as_bytes();
        let hashed_password = argon2::hash_encoded(
            password_bytes,
            self.config.argon_salt.as_bytes(),
            &Config::default(),
        )
        .unwrap();

        Ok(hashed_password)
    }

    fn verify_password(
        &self,
        stored_password: &str,
        attempted_password: String,
    ) -> AppResult<bool> {
        let hashes_match =
            argon2::verify_encoded(stored_password, attempted_password.as_bytes())
                .map_err(|err| AppError::InternalServerErrorWithContext(err.to_string()))?;

        Ok(hashes_match)
    }
}
