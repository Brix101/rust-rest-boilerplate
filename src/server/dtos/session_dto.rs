use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::database::session::Session;

impl Session {
    pub fn new(self) -> SessionResponseDto {
        SessionResponseDto {
            access_token: "access_token".to_string(),
            refresh_token: "refresh_token".to_string(),
        }
    }

    pub fn new_access_token(self) -> String {
        "access_token".to_string()
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SessionResponseDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct NewSessionDto {
    #[validate(required)]
    pub user_id: Option<Uuid>,
    #[validate(required, length(min = 1))]
    pub user_agent: Option<String>,
}
