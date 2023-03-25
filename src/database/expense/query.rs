use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::database::Database;

use super::repository::{Expense, ExpensesRepository};

#[async_trait]
impl ExpensesRepository for Database {
    async fn create_expense(
        &self,
        category_id: Uuid,
        amount: f64,
        description: String,
    ) -> anyhow::Result<Expense> {
        query_as!(
            Expense,
            r#"
        insert into expenses (created_at, updated_at, category_id, amount, description)
        values (current_timestamp, current_timestamp, $1, $2::float, $3::varchar)
        returning *
            "#,
            category_id,
            amount,
            description
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the expense")
    }

    async fn get_expense_by_id(&self, id: Uuid) -> anyhow::Result<Option<Expense>> {
        query_as!(
            Expense,
            r#"
        select *
        from expenses
        where id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("expense was not found")
    }

    async fn get_expenses(&self, user_id: Uuid) -> anyhow::Result<Vec<Expense>> {
        query_as!(
            Expense,
            r#"
        select expenses.*
        from expenses
        inner join categories on expenses.category_id=categories.id
        where categories.user_id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("expense was not found")
    }

    async fn update_expense(
        &self,
        id: Uuid,
        category_id: Uuid,
        amount: f64,
        description: String,
    ) -> anyhow::Result<Expense> {
        query_as!(
            Expense,
            r#"
        update expenses
        set
            category_id = $1,
            amount = $2::float,
            description = $3::varchar,
            updated_at = current_timestamp
        where id = $4
        returning *
            "#,
            category_id,
            amount,
            description,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the expense")
    }

    async fn delete_expense(&self, id: Uuid) -> anyhow::Result<()> {
        query!(
            r#"
        delete from expenses
        where id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting expense")?;

        Ok(())
    }
}
