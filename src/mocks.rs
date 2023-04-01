use crate::database::category::MockCategoriesRepository;
use crate::database::user::MockUsersRepository;
use crate::server::services::session_services::MockSessionsServiceTrait;
use crate::server::utils::argon_utils::MockArgonUtil;
use crate::server::utils::jwt_utils::MockJwtUtil;

pub struct CategoriesServiceTestFixture {
    pub mock_repository: MockCategoriesRepository,
}

impl CategoriesServiceTestFixture {
    pub fn new() -> Self {
        CategoriesServiceTestFixture {
            mock_repository: MockCategoriesRepository::new(),
        }
    }
}

impl Default for CategoriesServiceTestFixture {
    fn default() -> Self {
        CategoriesServiceTestFixture::new()
    }
}

pub struct UsersServiceTestFixture {
    pub mock_repository: MockUsersRepository,
    pub mock_jwt_util: MockJwtUtil,
    pub mock_argon_util: MockArgonUtil,
    pub mock_sessions_services: MockSessionsServiceTrait,
}

impl Default for UsersServiceTestFixture {
    fn default() -> Self {
        UsersServiceTestFixture::new()
    }
}

impl UsersServiceTestFixture {
    pub fn new() -> Self {
        Self {
            mock_repository: MockUsersRepository::new(),
            mock_jwt_util: MockJwtUtil::new(),
            mock_argon_util: MockArgonUtil::new(),
            mock_sessions_services: MockSessionsServiceTrait::new(),
        }
    }
}
