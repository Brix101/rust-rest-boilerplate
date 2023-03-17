use std::sync::Arc;

use async_trait::async_trait;
// use mockall::automock;
use super::user_repository::UserEntity;

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynUsersRepository = Arc<dyn SessionsRepository + Send + Sync>;

#[async_trait]
pub trait SessionsRepository {
    async fn create_session(&self, id: &i64, exp: &usize) -> anyhow::Result<UserEntity>;

    async fn get_user_session_by_id(&self, id: i64) -> anyhow::Result<UserEntity>;
}
