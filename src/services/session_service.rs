use sqlx::types::time::OffsetDateTime;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::info;

use async_trait::async_trait;

use crate::{
    dto::session_dto::{NewSessionDto, SessionResponseDto},
    repositories::session_repository::DynSessionsRepository,
    utils::{errors::AppResult, jwt_utils::DynJwtUtil},
};

/// A reference counter for our user service allows us safely pass instances user utils
/// around which themselves depend on the user repostiory, and ultimately, our Posgres connection pool.
pub type DynSessionsService = Arc<dyn SessionsServiceTrait + Send + Sync>;

#[async_trait]
pub trait SessionsServiceTrait {
    async fn new_session(&self, request: NewSessionDto) -> AppResult<SessionResponseDto>;

    // async fn new_access_token(
    //     &self,
    //     request: SignInUserDto,
    // ) -> AppResult<(ResponseUserDto, String)>;
}

#[derive(Clone)]
pub struct SessionsService {
    repository: DynSessionsRepository,
    jwt_util: DynJwtUtil,
}

impl SessionsService {
    pub fn new(repository: DynSessionsRepository, jwt_util: DynJwtUtil) -> Self {
        Self {
            repository,
            jwt_util,
        }
    }
}

#[async_trait]
impl SessionsServiceTrait for SessionsService {
    async fn new_session(&self, request: NewSessionDto) -> AppResult<SessionResponseDto> {
        let user_id = request.user_id.unwrap();
        let user_agent = request.user_agent.unwrap();
        let from_now = Duration::from_secs(604800);
        let expired_future_time = SystemTime::now().checked_add(from_now).unwrap();
        let exp = OffsetDateTime::from(expired_future_time);

        let created_session = self
            .repository
            .new_session(&user_id, user_agent.as_str(), &exp)
            .await?;

        let user_session = self
            .repository
            .get_user_session_by_id(created_session.id)
            .await?;

        info!("user successfully created, generating token");
        let access_token = self
            .jwt_util
            .new_access_token(user_session.id, &user_session.email)?;

        let refresh_token = self.jwt_util.new_refresh_token(created_session.id)?;

        Ok(SessionResponseDto {
            access_token,
            refresh_token,
        })
    }

    // async fn new_access_token(
    //     &self,
    //     request: SignInUserDto,
    // ) -> AppResult<(ResponseUserDto, String)> {

    // }
}
