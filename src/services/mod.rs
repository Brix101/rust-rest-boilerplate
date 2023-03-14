use std::sync::Arc;

use tracing::info;

use crate::{
    config::AppConfig,
    queries::user_query::UsersQuery,
    repositories::user_repository::DynUsersRepository,
    services::user_service::UsersService,
    utils::{
        connection_pool::ConnectionPool,
        jwt_utils::{DynTokenService, JwtService},
        password_util::{ArgonSecurityService, DynArgonService},
    },
};

use self::user_service::DynUsersService;

pub mod user_service;

#[derive(Clone)]
pub struct ServiceRegister {
    pub users_service: DynUsersService,
    pub token_service: DynTokenService,
}

impl ServiceRegister {
    pub fn new(pool: ConnectionPool, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");
        let security_service =
            Arc::new(ArgonSecurityService::new(config.clone())) as DynArgonService;
        let token_service = Arc::new(JwtService::new(config)) as DynTokenService;

        info!("utility services initialized, building feature services...");
        let users_repository = Arc::new(UsersQuery::new(pool.clone())) as DynUsersRepository;
        let users_service = Arc::new(UsersService::new(
            users_repository.clone(),
            security_service,
            token_service.clone(),
        )) as DynUsersService;

        ServiceRegister {
            users_service,
            token_service,
        }
    }
}
