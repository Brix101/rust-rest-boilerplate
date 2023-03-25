use anyhow::Context;
use async_trait::async_trait;
use sqlx::query_as;
use uuid::Uuid;

use crate::database::Database;

use super::{User, UsersRepository};

#[async_trait]
impl UsersRepository for Database {
    async fn create_user(
        &self,
        email: &str,
        name: &str,
        hash_password: &str,
    ) -> anyhow::Result<User> {
        query_as!(
            User,
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

    async fn get_user_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
        query_as!(
            User,
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

    async fn get_user_by_id(&self, id: Uuid) -> anyhow::Result<User> {
        query_as!(
            User,
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
        id: Uuid,
        email: String,
        name: String,
        password: String,
        bio: String,
        image: String,
    ) -> anyhow::Result<User> {
        query_as!(
            User,
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
