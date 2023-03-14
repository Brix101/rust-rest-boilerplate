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
    pub token: String,
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
        token: String,
    ) -> Self {
        UserAuthenicationResponse {
            user: ResponseUserDto {
                id,
                name,
                email,
                bio,
                image,
                token,
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

#[derive(Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterUserRequest {
    #[validate]
    pub user: RegisterUserDto,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginUserRequest {
    #[validate]
    pub user: LoginUserDto,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateUserRequest {
    pub user: UpdateUserDto,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterUserDto {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct LoginUserDto {
    #[validate(required, length(min = 1), email(message = "email is invalid"))]
    pub email: Option<String>,
    #[validate(required, length(min = 1))]
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

impl RegisterUserDto {
    pub fn new_stub() -> Self {
        Self {
            name: Some(String::from("stub name")),
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}

impl LoginUserDto {
    pub fn new_stub() -> Self {
        Self {
            email: Some(String::from("stub email")),
            password: Some(String::from("stub password")),
        }
    }
}
