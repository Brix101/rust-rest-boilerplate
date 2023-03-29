// use std::str::FromStr;
use std::{sync::Arc, time::SystemTime};
// use std::time::SystemTime;

use async_trait::async_trait;
use mockall::automock;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;
use uuid::{uuid, Uuid};

/// Similar to above, we want to keep a reference count across threads so we can manage our connection pool.
pub type DynCategoriesRepository = Arc<dyn CategoriesRepository + Send + Sync>;

#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[sqlx(type_name = "category_type")]
pub enum CategoryType {
    Essential,
    NonEssential,
}

impl Default for CategoryType {
    fn default() -> Self {
        Self::NonEssential
    }
}

#[automock]
#[async_trait]
pub trait CategoriesRepository {
    async fn create_category(
        &self,
        user_id: Uuid,
        name: String,
        cat_type: CategoryType,
    ) -> anyhow::Result<Category>;

    async fn get_category_by_id(&self, id: Uuid) -> anyhow::Result<Option<Category>>;

    async fn get_categories(&self, user_id: Uuid) -> anyhow::Result<Vec<Category>>;

    async fn update_category(
        &self,
        id: Uuid,
        name: String,
        cat_type: CategoryType,
    ) -> anyhow::Result<Category>;

    async fn delete_category(&self, id: Uuid) -> anyhow::Result<()>;
}

#[derive(FromRow, Debug)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub cat_type: CategoryType,
    pub user_id: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Default for Category {
    fn default() -> Self {
        Self {
            id: uuid!("b7f9ddc7-c80d-4bf6-8573-f06e94addfb3"),
            name: String::from("stub category"),
            cat_type: CategoryType::default(),
            user_id: uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"),
            created_at: OffsetDateTime::from(SystemTime::now()),
            updated_at: OffsetDateTime::from(SystemTime::now()),
        }
    }
}
