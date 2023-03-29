use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::database::Database;

use super::repository::{CategoriesRepository, Category, CategoryType};

#[async_trait]
impl CategoriesRepository for Database {
    async fn create_category(
        &self,
        user_id: Uuid,
        name: String,
        cat_type: CategoryType,
    ) -> anyhow::Result<Category> {
        query_as!(
            Category,
            r#"
        insert into categories (created_at, updated_at, name, user_id,cat_type)
        values (current_timestamp, current_timestamp, $1::varchar, $2, $3)
        returning id, name, cat_type as "cat_type: CategoryType", user_id, created_at, updated_at
            "#,
            name,
            user_id,
            cat_type as _
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the category")
    }

    async fn get_category_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>> {
        query_as!(
            Category,
            r#"
        select id, name, cat_type as "cat_type: CategoryType", user_id, created_at, updated_at
        from categories
        where id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("category was not found")
    }

    async fn get_categories(&self, user_id: Uuid) -> anyhow::Result<Vec<Category>> {
        query_as!(
            Category,
            r#"
        select categories.id, categories.name, categories.cat_type as "cat_type: CategoryType",
        categories.user_id, categories.created_at, categories.updated_at
        from categories
        inner join users on categories.user_id=users.id
        where users.id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("category was not found")
    }

    async fn update_category(
        &self,
        id: Uuid,
        name: String,
        cat_type: CategoryType,
    ) -> anyhow::Result<Category> {
        query_as!(
            Category,
            r#"
        update categories
        set
            name = $1::varchar,
            cat_type = $2
        where id = $3
        returning id, name, cat_type as "cat_type: CategoryType", user_id, created_at, updated_at
            "#,
            name,
            cat_type as _,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the category")
    }

    async fn delete_category(&self, id: Uuid) -> anyhow::Result<()> {
        query!(
            r#"
        delete from categories
        where id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting category")?;

        Ok(())
    }
}
