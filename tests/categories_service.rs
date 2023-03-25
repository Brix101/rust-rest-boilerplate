use std::sync::Arc;

use budgetto_api::{
    database::category::repository::{Category, CategoryType, DynCategoriesRepository},
    mocks::CategoriesServiceTestFixture,
    server::{
        dtos::category_dto::CategoryCreateDto,
        services::category_services::{CategoriesService, CategoriesServiceTrait},
    },
};
use mockall::predicate::*;
use uuid::uuid;

#[tokio::test]
async fn return_success_when_category_created() {
    // arrange
    let mut fixture = CategoriesServiceTestFixture::default();

    fixture
        .mock_repository
        .expect_create_category()
        .with(
            eq(uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e")),
            eq(String::from("stub category")),
            eq(CategoryType::default()),
        )
        .times(1)
        .return_once(move |_, _, _| Ok(Category::default()));

    let categories_service =
        CategoriesService::new(Arc::new(fixture.mock_repository) as DynCategoriesRepository);

    // act
    let response = categories_service
        .create_category(
            uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e"),
            CategoryCreateDto::new_stub(),
        )
        .await;

    // assert
    assert!(response.is_ok());
}
