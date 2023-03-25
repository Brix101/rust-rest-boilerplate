mod category_api;
mod user_api;

use axum::routing::*;

use category_api::CategoryRouter;
use user_api::UsersRouter;

pub async fn health() -> &'static str {
    "🚀🚀🚀 Server Running"
}

pub fn app() -> Router {
    Router::new()
        .nest("/users", UsersRouter::app())
        .nest("/categories", CategoryRouter::app())
        .route("/health", get(health))
}
