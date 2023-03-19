use axum::extract::Json;
use axum::routing::{get, post, put};
use axum::{Extension, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use tracing::info;

use crate::dto::user_dto::{
    SignInUserDto, SignUpUserDto, UpdateUserDto, UserAuthenicationResponse,
};
use crate::middlewares::request_validation_middleware::ValidatedRequest;
use crate::middlewares::required_authentication_middleware::RequiredAuthentication;
use crate::services::user_service::DynUsersService;
use crate::services::ServiceRegister;
use crate::utils::errors::AppResult;

pub struct UsersRouter;

impl UsersRouter {
    pub fn new_router(service_register: ServiceRegister) -> Router {
        Router::new()
            .route("/signup", post(UsersRouter::signup_user_endpoint))
            .route("/signin", post(UsersRouter::signin_user_endpoint))
            .route("/user", get(UsersRouter::get_current_user_endpoint))
            .route("/user", put(UsersRouter::update_user_endpoint))
            .layer(Extension(service_register.users_service))
            .layer(Extension(service_register.token_service))
    }

    pub async fn signup_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        ValidatedRequest(request): ValidatedRequest<SignUpUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to create user {:?}/{:?}",
            request.email.as_ref().unwrap(),
            request.name.as_ref().unwrap()
        );

        let created_user = users_service.signup_user(request).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }
    pub async fn signin_user_endpoint(
        jar: CookieJar,
        Extension(users_service): Extension<DynUsersService>,
        ValidatedRequest(request): ValidatedRequest<SignInUserDto>,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!(
            "recieved request to login user {:?}",
            request.email.as_ref().unwrap()
        );

        let (user, refresh_token) = users_service.signin_user(request).await?;

        let cookie = jar.add(Cookie::new("refresh_token", refresh_token.to_string()));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
    }

    pub async fn get_current_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        RequiredAuthentication(user_id): RequiredAuthentication,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to retrieve current user");

        let current_user = users_service.get_current_user(user_id).await?;

        Ok(Json(UserAuthenicationResponse { user: current_user }))
    }

    pub async fn update_user_endpoint(
        Extension(users_service): Extension<DynUsersService>,
        RequiredAuthentication(user_id): RequiredAuthentication,
        Json(request): Json<UpdateUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to update user {:?}", user_id);

        let updated_user = users_service.updated_user(user_id, request).await?;

        Ok(Json(UserAuthenicationResponse { user: updated_user }))
    }
}
