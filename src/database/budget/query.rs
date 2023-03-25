use anyhow::Context;
use async_trait::async_trait;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::database::Database;

use super::repository::{Budget, BudgetsRepository, PlanType};

#[async_trait]
impl BudgetsRepository for Database {
    async fn create_budget(
        &self,
        category_id: Uuid,
        amount: f64,
        description: String,
        plan: PlanType,
    ) -> anyhow::Result<Budget> {
        query_as!(
            Budget,
            r#"
        insert into budgets (created_at, updated_at, category_id, amount, description,plan)
        values (current_timestamp, current_timestamp, $1, $2::float, $3::varchar, $4)
        returning id, amount, description, category_id, plan as "plan: PlanType", created_at, updated_at, deleted_at
            "#,
            category_id,
            amount,
            description,
            plan as _
        )
        .fetch_one(&self.pool)
        .await
        .context("an unexpected error occured while creating the budget")
    }

    async fn get_budget_by_id(&self, id: Uuid) -> anyhow::Result<Option<Budget>> {
        query_as!(
            Budget,
            r#"
        select id, amount, description, category_id, plan as "plan: PlanType", created_at, updated_at, deleted_at
        from budgets
        where id = $1
            "#,
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .context("budget was not found")
    }

    async fn get_budgets(&self, user_id: Uuid) -> anyhow::Result<Vec<Budget>> {
        query_as!(
            Budget,
            r#"
        select budgets.id, budgets.amount, budgets.description, budgets.category_id, budgets.plan as "plan: PlanType",
        budgets.created_at, budgets.updated_at, budgets.deleted_at
        from budgets
        inner join categories on categories.id = budgets.id
        where categories.user_id = $1
            "#,
            user_id,
        )
        .fetch_all(&self.pool)
        .await
        .context("budget was not found")
    }

    async fn update_budget(
        &self,
        id: Uuid,
        category_id: Uuid,
        amount: f64,
        description: String,
        plan: PlanType,
    ) -> anyhow::Result<Budget> {
        query_as!(
            Budget,
            r#"
        update budgets
        set
            category_id = $1,
            amount = $2::float,
            description = $3::varchar,
            plan = $4,
            updated_at = current_timestamp
        where id = $5
        returning id, amount, description, category_id, plan as "plan: PlanType", created_at, updated_at, deleted_at
            "#,
            category_id,
            amount,
            description,
            plan as _,
            id
        )
        .fetch_one(&self.pool)
        .await
        .context("could not update the budget")
    }

    async fn delete_budget(&self, id: Uuid) -> anyhow::Result<()> {
        query!(
            r#"
        delete from budgets
        where id = $1
        "#,
            id
        )
        .execute(&self.pool)
        .await
        .context("an unexpected error occurred deleting budget")?;

        Ok(())
    }
}
