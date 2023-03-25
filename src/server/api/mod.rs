mod budget_api;
mod category_api;
mod expense_api;
mod user_api;

use axum::routing::*;

use budget_api::BudgetRouter;
use category_api::CategoryRouter;
use expense_api::ExpenseRouter;
use user_api::UsersRouter;

pub async fn health() -> &'static str {
    "🚀🚀🚀 Server Running"
}

pub fn app() -> Router {
    Router::new()
        .nest("/users", UsersRouter::app())
        .nest("/budgets", BudgetRouter::app())
        .nest("/categories", CategoryRouter::app())
        .nest("/expenses", ExpenseRouter::app())
        .route("/health", get(health))
}
