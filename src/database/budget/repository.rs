use std::sync::Arc;
use std::time::SystemTime;

use async_trait::async_trait;
use mockall::automock;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::{uuid, Uuid};

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynBudgetsRepository = Arc<dyn BudgetsRepository + Send + Sync>;

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "plan_type")]
pub enum PlanType {
    Daily,
    Weekly,
    Monthly,
}

impl Default for PlanType {
    fn default() -> Self {
        Self::Monthly
    }
}

#[automock]
#[async_trait]
pub trait BudgetsRepository {
    async fn create_budget(
        &self,
        category_id: Uuid,
        amount: f64,
        description: String,
        plan: PlanType,
    ) -> anyhow::Result<Budget>;

    async fn get_budget_by_id(&self, id: Uuid) -> anyhow::Result<Option<Budget>>;

    async fn get_budgets(&self, user_id: Uuid) -> anyhow::Result<Vec<Budget>>;

    async fn update_budget(
        &self,
        id: Uuid,
        category_id: Uuid,
        amount: f64,
        description: String,
        plan: PlanType,
    ) -> anyhow::Result<Budget>;

    async fn delete_budget(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Budget {
    pub id: Uuid,
    pub amount: f64,
    pub description: String,
    pub plan: PlanType,
    pub category_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl Default for Budget {
    fn default() -> Self {
        Self {
            id: uuid!("7b684cc4-e636-4d41-ac54-43e673aa9a60"),
            amount: 99999_f64,
            category_id: uuid!("b7f9ddc7-c80d-4bf6-8573-f06e94addfb3"),
            description: String::from("stub expense description"),
            plan: PlanType::default(),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            deleted_at: Some(OffsetDateTime::from(SystemTime::now())),
        }
    }
}
