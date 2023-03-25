use axum::extract::{Json, Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use tracing::info;
use uuid::Uuid;

use crate::server::dtos::budget_dto::{
    BudgetCreateDto, BudgetQuery, BudgetResponseDto, BudgetUpdateDto,
};
use crate::server::error::AppResult;
use crate::server::middlewares::{RequiredAuthentication, ValidatedRequest};
use crate::server::services::Services;

pub struct BudgetRouter;

impl BudgetRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::get_user_budgets))
            .route("/", post(Self::create_budget))
            .route("/:id", put(Self::update_budget))
            .route("/:id", delete(Self::delete_budget))
    }

    pub async fn get_user_budgets(
        query_params: Query<BudgetQuery>,
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
    ) -> AppResult<Json<Vec<BudgetResponseDto>>> {
        info!("received request to get current user budgets");

        if let Some(id) = query_params.budget_id {
            // return this function if the query params has value
            let budget = services.budgets.get_budget_by_id(id, user_id).await?;

            return Ok(Json(vec![budget]));
        }

        let budgets = services.budgets.get_budgets(user_id).await?;

        Ok(Json(budgets))
    }

    pub async fn create_budget(
        Extension(services): Extension<Services>,
        RequiredAuthentication(_user_id): RequiredAuthentication,
        ValidatedRequest(request): ValidatedRequest<BudgetCreateDto>,
    ) -> AppResult<Json<BudgetResponseDto>> {
        info!("received request to create budget");

        let new_budget = services.budgets.create_budget(request).await?;

        Ok(Json(new_budget))
    }

    pub async fn update_budget(
        Path(id): Path<Uuid>,
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
        Json(request): Json<BudgetUpdateDto>,
    ) -> AppResult<Json<BudgetResponseDto>> {
        info!("recieved request to update budget {:?}", id);

        let updated_budget = services
            .budgets
            .updated_budget(id, user_id, request)
            .await?;

        Ok(Json(updated_budget))
    }

    pub async fn delete_budget(
        Path(id): Path<Uuid>,
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
    ) -> AppResult<()> {
        info!("recieved request to remove budget {:?}", id);

        services.budgets.delete_budget(user_id, id).await?;

        Ok(())
    }
}
