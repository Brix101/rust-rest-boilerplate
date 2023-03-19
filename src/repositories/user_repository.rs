use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
// use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

use crate::dto::user_dto::{ResponseUserDto, UserProfileDto};

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[async_trait]
pub trait UsersRepository {
    async fn create_user(
        &self,
        email: &str,
        name: &str,
        hash_password: &str,
    ) -> anyhow::Result<UserEntity>;

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserEntity>>;

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<UserEntity>;

    async fn update_user(
        &self,
        id: i64,
        email: String,
        name: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<UserEntity>;
}

#[derive(FromRow)]
pub struct UserEntity {
    pub id: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub bio: String,
    pub image: String,
}

impl UserEntity {
    pub fn into_dto(self, token: String) -> ResponseUserDto {
        ResponseUserDto {
            id: self.id,
            email: self.email,
            name: self.name,
            bio: Some(self.bio),
            image: Some(self.image),
            access_token: token,
        }
    }

    pub fn into_profile(self, following: bool) -> UserProfileDto {
        UserProfileDto {
            name: self.name,
            bio: self.bio,
            image: self.image,
            following,
        }
    }
}

impl Default for UserEntity {
    fn default() -> Self {
        UserEntity {
            id: 1,
            bio: String::from("stub bio"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            deleted_at: Some(OffsetDateTime::from(SystemTime::now())),
            name: String::from("stub name"),
            email: String::from("stub email"),
            password: String::from("hashed password"),
            image: String::from("stub image"),
        }
    }
}
