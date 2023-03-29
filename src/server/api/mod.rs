mod category_controller;
mod user_controller;

use axum::routing::*;

use self::{category_controller::CategoryController, user_controller::UserController};

pub async fn health() -> &'static str {
    "🚀🚀🚀 Server Running"
}

pub fn app() -> Router {
    Router::new()
        .nest("/users", UserController::app())
        .nest("/categories", CategoryController::app())
        .route("/health", get(health))
}
