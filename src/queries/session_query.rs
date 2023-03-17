use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;

use crate::repositories::user_repository::{UserEntity, UsersRepository};
use crate::utils::connection_pool::ConnectionPool;

#[derive(Clone)]
pub struct UsersQuery {
    pool: ConnectionPool,
}

impl UsersQuery {
    pub fn new(pool: ConnectionPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UsersRepository for UsersQuery {
    async fn create_user(
        &self,
        email: &str,
        name: &str,
        hash_password: &str,
    ) -> anyhow::Result<UserEntity> {
        query_as!(
            UserEntity,
            r#"
        insert into users (created_at, updated_at, name, email, password, bio, image)
        values (current_timestamp, current_timestamp, $1::varchar, $2::varchar, $3::varchar, '', '')
        returning *
            "#,
            name,
            email,
            hash_password
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the user")
    }

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<UserEntity>> {
        query_as!(
            UserEntity,
            r#"
        select *
        from users
        where email = $1::varchar
            "#,
            email,
        )
        .fetch_optional(&self.pool)
        .await
        .context("unexpected error while querying for user by email")
    }

    async fn get_user_by_id(&self, id: i64) -> anyhow::Result<UserEntity> {
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

    async fn update_user(
        &self,
        id: i64,
        email: String,
        name: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<UserEntity> {
        query_as!(
            UserEntity,
            r#"
        update users
        set
            name = $1::varchar,
            email = $2::varchar,
            password = $3::varchar,
            bio = $4::varchar,
            image = $5::varchar,
            updated_at = current_timestamp
        where id = $6
        returning *
            "#,
            name,
            email,
            password,
            bio,
            image,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the user")
    }
}
