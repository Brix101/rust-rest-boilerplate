use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::{uuid, Uuid};

#[derive(FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"),
            email: String::from("stub email"),
            name: String::from("stub name"),
            password: String::from("hashed password"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait UsersRepository {
    async fn create_user(
        &self,
        email: &str,
        name: &str,
        hash_password: &str,
    ) -> anyhow::Result<User>;

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;

    async fn get_user_by_id(&self, id: Uuid) -> anyhow::Result<User>;

    async fn update_user(
        &self,
        id: Uuid,
        email: String,
        name: String,
        password: String,
    ) -> anyhow::Result<User>;
}
