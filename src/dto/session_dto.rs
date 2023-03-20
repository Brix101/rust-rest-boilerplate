use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SessionResponseDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct NewSessionDto {
    #[validate(required)]
    pub user_id: Option<i64>,
    #[validate(required, length(min = 1))]
    pub user_agent: Option<String>,
}
