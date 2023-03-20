use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResponseUserDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i64,
    pub name: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub access_token: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserAuthenicationResponse {
    pub user: ResponseUserDto,
}

impl UserAuthenicationResponse {
    pub fn new(
        id: i64,
        name: String,
        email: String,
        // unfortunately, while our implementation returns thes optional fields as empty strings,
        // the realworld demo API enables nullable serializing by default, so we have to wrap these
        // strings as `Option` option values for now
        bio: Option<String>,
        image: Option<String>,
        access_token: Option<String>,
    ) -> Self {
        UserAuthenicationResponse {
            user: ResponseUserDto {
                id,
                name,
                email,
                bio,
                image,
                access_token,
            },
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct UserProfileDto {
    pub name: String,
    pub bio: String,
    pub image: String,
    pub following: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct SignUpUserDto {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 6))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct SignInUserDto {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 6))]
    pub password: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl SignUpUserDto {
    pub fn new_stub() -> Self {
        Self {
            name: Some(String::from("stub name")),
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}

impl SignInUserDto {
    pub fn new_stub() -> Self {
        Self {
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}
