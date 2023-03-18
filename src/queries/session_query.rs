use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;
use sqlx::types::time::OffsetDateTime;

use crate::repositories::session_repository::{SessionEntity, SessionsRepository};
use crate::repositories::user_repository::UserEntity;
use crate::utils::connection_pool::ConnectionPool;

#[derive(Clone)]
pub struct SessionsQuery {
    pool: ConnectionPool,
}

impl SessionsQuery {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionsRepository for SessionsQuery {
    async fn create_session(
        &self,
        user_id: &i64,
        user_agent: &str,
        exp: &OffsetDateTime,
    ) -> anyhow::Result<SessionEntity> {
        query_as!(
            SessionEntity,
            r#"
        insert into user_sessions (user_id,user_agent,exp)
        values ($1,$2,'2023-03-17 09:02:37.447991+00')
        returning *
            "#,
            user_id,
            user_agent,
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating a session")
    }

    async fn get_user_session_by_id(&self, id: i64) -> anyhow::Result<UserEntity> {
        query_as!(
            UserEntity,
            r#"
        select *
        from users
        where id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await
        .context("user was not found")
    }
}
