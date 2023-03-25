use std::sync::Arc;

use budgetto_api::{
    database::user::{DynUsersRepository, User},
    mocks::UsersServiceTestFixture,
    server::{
        dtos::user_dto::SignUpUserDto,
        services::{
            session_services::DynSessionsService,
            user_services::{UsersService, UsersServiceTrait},
        },
        utils::{argon_utils::DynArgonUtil, jwt_utils::DynJwtUtil},
    },
};
use mockall::predicate::*;
use uuid::uuid;

#[tokio::test]
async fn return_success_when_downstream_services_succeed() {
    // arrange
    let mut fixture = UsersServiceTestFixture::default();

    fixture
        .mock_repository
        .expect_get_user_by_email()
        .with(eq("stub email"))
        .times(1)
        .return_once(move |_| Ok(None));

    fixture
        .mock_repository
        .expect_create_user()
        .with(eq("stub email"), eq("stub name"), eq("hashed password"))
        .times(1)
        .return_once(move |_, _, _| Ok(User::default()));

    fixture
        .mock_argon_util
        .expect_hash_password()
        .with(eq("stub password"))
        .times(1)
        .return_once(move |_| Ok(String::from("hashed password")));

    fixture
        .mock_jwt_util
        .expect_new_access_token()
        .with(
            eq(uuid!("f3f898aa-ffa3-4b58-91b0-612a1c801a5e")),
            eq("stub email"),
        )
        .times(0)
        .return_once(move |_, _| Ok(String::from("stub token")));

    let users_service = UsersService::new(
        Arc::new(fixture.mock_repository) as DynUsersRepository,
        Arc::new(fixture.mock_argon_util) as DynArgonUtil,
        Arc::new(fixture.mock_jwt_util) as DynJwtUtil,
        Arc::new(fixture.mock_sessions_services) as DynSessionsService,
    );

    // act
    let response = users_service.signup_user(SignUpUserDto::new_stub()).await;

    // assert
    assert!(response.is_ok());
}

#[tokio::test]
async fn return_error_when_user_exixsts() {
    // arrange
    let mut fixture = UsersServiceTestFixture::default();

    fixture
        .mock_repository
        .expect_get_user_by_email()
        .with(eq("stub email"))
        .times(1)
        .return_once(move |_| Ok(Some(User::default())));

    fixture.mock_repository.expect_create_user().times(0);

    fixture.mock_argon_util.expect_hash_password().times(0);

    fixture.mock_jwt_util.expect_new_access_token().times(0);

    let users_service = UsersService::new(
        Arc::new(fixture.mock_repository) as DynUsersRepository,
        Arc::new(fixture.mock_argon_util) as DynArgonUtil,
        Arc::new(fixture.mock_jwt_util) as DynJwtUtil,
        Arc::new(fixture.mock_sessions_services) as DynSessionsService,
    );

    // act
    let response = users_service.signup_user(SignUpUserDto::new_stub()).await;

    // assert
    assert!(response.is_err());
}
