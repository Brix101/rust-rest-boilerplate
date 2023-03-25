use axum::extract::{Json, Path, Query};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use tracing::info;
use uuid::Uuid;

use crate::server::dtos::category_dto::{
    CategoryCreateDto, CategoryQuery, CategoryResponseDto, CategoryUpdateDto,
};
use crate::server::error::AppResult;
use crate::server::middlewares::{RequiredAuthentication, ValidatedRequest};
use crate::server::services::Services;

pub struct CategoryRouter;

impl CategoryRouter {
    pub fn app() -> Router {
        Router::new()
            .route("/", get(Self::get_user_categories))
            .route("/", post(Self::create_category))
            .route("/:id", put(Self::update_category))
            .route("/:id", delete(Self::delete_category))
    }

    pub async fn get_user_categories(
        query_params: Query<CategoryQuery>,
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
    ) -> AppResult<Json<Vec<CategoryResponseDto>>> {
        info!("received request to get current user categorys");

        if let Some(id) = query_params.category_id {
            // return this function if the query params has value
            let category = services.categories.get_category_by_id(id, user_id).await?;

            return Ok(Json(vec![category]));
        }

        let categories = services.categories.get_categories(user_id).await?;

        Ok(Json(categories))
    }

    pub async fn create_category(
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
        ValidatedRequest(request): ValidatedRequest<CategoryCreateDto>,
    ) -> AppResult<Json<CategoryResponseDto>> {
        info!("received request to create category");

        let new_category = services
            .categories
            .create_category(user_id, request)
            .await?;

        Ok(Json(new_category))
    }

    pub async fn update_category(
        Path(id): Path<Uuid>,
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
        Json(request): Json<CategoryUpdateDto>,
    ) -> AppResult<Json<CategoryResponseDto>> {
        info!("recieved request to update category {:?}", id);

        let updated_category = services
            .categories
            .updated_category(id, user_id, request)
            .await?;

        Ok(Json(updated_category))
    }

    pub async fn delete_category(
        Path(id): Path<Uuid>,
        Extension(services): Extension<Services>,
        RequiredAuthentication(user_id): RequiredAuthentication,
    ) -> AppResult<()> {
        info!("recieved request to remove category {:?}", id);

        services.categories.delete_category(user_id, id).await?;

        Ok(())
    }
}
