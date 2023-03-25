use axum::extract::{Json, Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use tracing::info;
use uuid::Uuid;

use crate::server::dtos::expense_dto::{
    ExpenseCreateDto, ExpenseQuery, ExpenseResponseDto, ExpenseUpdateDto,
};
use crate::server::error::AppResult;
use crate::server::middlewares::{RequiredAuthentication, ValidatedRequest};
use crate::server::services::Services;

pub struct ExpenseRouter;

impl ExpenseRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::get_user_expenses))
            .route("/", post(Self::create_expense))
            .route("/:id", put(Self::update_expense))
            .route("/:id", delete(Self::delete_expense))
    }

    pub async fn get_user_expenses(
        query_params: Query<ExpenseQuery>,
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
    ) -> AppResult<Json<Vec<ExpenseResponseDto>>> {
        info!("received request to get current user expenses");

        if let Some(id) = query_params.expense_id {
            // return this function if the query params has value
            let expense = services.expenses.get_expense_by_id(id, user_id).await?;

            return Ok(Json(vec![expense]));
        }

        let expenses = services.expenses.get_expenses(user_id).await?;

        Ok(Json(expenses))
    }

    pub async fn create_expense(
        Extension(services): Extension<Services>,
        RequiredAuthentication(_user_id): RequiredAuthentication,
        ValidatedRequest(request): ValidatedRequest<ExpenseCreateDto>,
    ) -> AppResult<Json<ExpenseResponseDto>> {
        info!("received request to create expense");

        let new_expense = services.expenses.create_expense(request).await?;

        Ok(Json(new_expense))
    }

    pub async fn update_expense(
        Path(id): Path<Uuid>,
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
        Json(request): Json<ExpenseUpdateDto>,
    ) -> AppResult<Json<ExpenseResponseDto>> {
        info!("recieved request to update expense {:?}", id);

        let updated_expense = services
            .expenses
            .updated_expense(id, user_id, request)
            .await?;

        Ok(Json(updated_expense))
    }

    pub async fn delete_expense(
        Path(id): Path<Uuid>,
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
    ) -> AppResult<()> {
        info!("recieved request to remove expense {:?}", id);

        services.expenses.delete_expense(user_id, id).await?;

        Ok(())
    }
}
