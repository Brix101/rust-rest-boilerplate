use std::{sync::Arc, time::SystemTime};

use async_trait::async_trait;
use mockall::automock;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::{uuid, Uuid};

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynExpensesRepository = Arc<dyn ExpensesRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait ExpensesRepository {
    async fn create_expense(
        &self,
        category_id: Uuid,
        amount: f64,
        description: String,
    ) -> anyhow::Result<Expense>;

    async fn get_expense_by_id(&self, id: Uuid) -> anyhow::Result<Option<Expense>>;

    async fn get_expenses(&self, user_id: Uuid) -> anyhow::Result<Vec<Expense>>;

    async fn update_expense(
        &self,
        id: Uuid,
        category_id: Uuid,
        amount: f64,
        description: String,
    ) -> anyhow::Result<Expense>;

    async fn delete_expense(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Expense {
    pub id: Uuid,
    pub category_id: Uuid,
    pub amount: f64,
    pub description: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

impl Default for Expense {
    fn default() -> Self {
        Self {
            id: uuid!("ef234da0-9001-4313-9686-2ab204a10223"),
            amount: 99999_f64,
            category_id: uuid!("b7f9ddc7-c80d-4bf6-8573-f06e94addfb3"),
            description: String::from("stub expense description"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
            deleted_at: Some(OffsetDateTime::from(SystemTime::now())),
        }
    }
}
