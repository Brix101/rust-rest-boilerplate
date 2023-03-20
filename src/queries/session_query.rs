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
    async fn new_session(
        &self,
        user_id: &i64,
        user_agent: &str,
        exp: &OffsetDateTime,
    ) -> anyhow::Result<SessionEntity> {
        query_as!(
            SessionEntity,
            r#"
        insert into user_sessions (user_id,user_agent,exp)
        values ($1,$2,$3)
        returning *
            "#,
            user_id,
            user_agent,
            exp
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating a session")
    }

    async fn get_user_session_by_id(&self, id: i64) -> anyhow::Result<UserEntity> {
        query_as!(
            UserEntity,
            r#"
        select users.* from users
        inner join user_sessions
        on users.id = user_sessions.user_id
        where user_sessions.id = $1
            "#,
            id,
        )
        .fetch_one(&self.pool)
        .await
        .context("user was not found")
    }
}
