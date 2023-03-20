use std::sync::Arc;

use tracing::info;

use crate::{
    config::AppConfig,
    queries::{session_query::SessionsQuery, user_query::UsersQuery},
    repositories::{
        session_repository::DynSessionsRepository, user_repository::DynUsersRepository,
    },
    services::{session_service::SessionsService, user_service::UsersService},
    utils::{
        argon_util::{ArgonSecurityUtil, DynArgonUtil},
        connection_pool::ConnectionPool,
        jwt_utils::{DynJwtUtil, JwtTokenUtil},
    },
};

use self::{session_service::DynSessionsService, user_service::DynUsersService};

pub mod session_service;
pub mod user_service;

#[derive(Clone)]
pub struct ServiceRegister {
    pub users_service: DynUsersService,
    pub jwt_util: DynJwtUtil,
    pub sessions_service: DynSessionsService,
}

impl ServiceRegister {
    pub fn new(pool: ConnectionPool, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");
        let security_service = Arc::new(ArgonSecurityUtil::new(config.clone())) as DynArgonUtil;
        let jwt_util = Arc::new(JwtTokenUtil::new(config)) as DynJwtUtil;

        info!("utility services initialized, building feature services...");
        let users_repository = Arc::new(UsersQuery::new(pool.clone())) as DynUsersRepository;

        let session_repository =
            Arc::new(SessionsQuery::new(pool.clone())) as DynSessionsRepository;

        let sessions_service = Arc::new(SessionsService::new(
            session_repository.clone(),
            jwt_util.clone(),
        )) as DynSessionsService;

        let users_service = Arc::new(UsersService::new(
            users_repository.clone(),
            security_service,
            jwt_util.clone(),
            sessions_service.clone(),
        )) as DynUsersService;

        ServiceRegister {
            users_service,
            jwt_util,
            sessions_service,
        }
    }
}
