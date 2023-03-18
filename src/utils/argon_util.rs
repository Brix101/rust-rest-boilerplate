use std::sync::Arc;

use argon2::Config;

use crate::config::AppConfig;

use super::errors::{AppError, AppResult};
// use mockall::automock;

/// A security service for handling JWT authentication.
pub type DynArgonUtil = Arc<dyn ArgonUtil + Send + Sync>;

// #[automock]
pub trait ArgonUtil {
    fn hash_password(&self, raw_password: &str) -> AppResult<String>;

    fn verify_password(&self, stored_password: &str, attempted_password: String)
        -> AppResult<bool>;
}

pub struct ArgonSecurityUtil {
    config: Arc<AppConfig>,
}

impl ArgonSecurityUtil {
    pub fn new(config: Arc<AppConfig>) -> Self {
        Self { config }
    }
}

impl ArgonUtil for ArgonSecurityUtil {
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
