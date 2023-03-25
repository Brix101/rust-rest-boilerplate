use std::sync::Arc;

use tracing::info;

use crate::{
    config::AppConfig,
    database::Database,
    server::{
        services::{
            category_services::CategoriesService, session_services::SessionsService,
            user_services::UsersService,
        },
        utils::{
            argon_utils::{ArgonSecurityUtil, DynArgonUtil},
            jwt_utils::JwtTokenUtil,
        },
    },
};

use self::{
    category_services::DynCategoriesService, session_services::DynSessionsService,
    user_services::DynUsersService,
};

use super::utils::jwt_utils::DynJwtUtil;

pub mod category_services;
pub mod seed_services;
pub mod session_services;
pub mod user_services;

#[derive(Clone)]
pub struct Services {
    pub jwt_util: DynJwtUtil,
    pub users: DynUsersService,
    pub sessions: DynSessionsService,
    pub categories: DynCategoriesService,
}

impl Services {
    pub fn new(db: Database, config: Arc<AppConfig>) -> Self {
        info!("initializing utility services...");
        let security_service = Arc::new(ArgonSecurityUtil::new(config.clone())) as DynArgonUtil;
        let jwt_util = Arc::new(JwtTokenUtil::new(config)) as DynJwtUtil;

        info!("utility services initialized, building feature services...");
        let repository = Arc::new(db);

        let sessions = Arc::new(SessionsService::new(repository.clone(), jwt_util.clone()))
            as DynSessionsService;

        let users = Arc::new(UsersService::new(
            repository.clone(),
            security_service,
            jwt_util.clone(),
            sessions.clone(),
        )) as DynUsersService;

        let categories =
            Arc::new(CategoriesService::new(repository.clone())) as DynCategoriesService;

        Self {
            jwt_util,
            users,
            sessions,
            categories,
        }
    }
}
