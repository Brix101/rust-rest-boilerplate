use lazy_static::lazy_static;
use tracing::info;

use crate::{
    database::category::repository::CategoryType,
    server::{
        dtos::{
            category_dto::CategoryCreateDto,
            user_dto::{ResponseUserDto, SignInUserDto, SignUpUserDto},
        },
        error::AppResult,
    },
};

use super::{category_services::DynCategoriesService, user_services::DynUsersService, Services};

lazy_static! {
    static ref TEST_USER_1_NAME: &'static str = "testuser1";
    static ref TEST_USER_1_EMAIL: &'static str = "testuser1@gmail.com";
    static ref TEST_USER_1_PASSWORD: &'static str = "password";
    static ref TEST_USER_2_NAME: &'static str = "testuser2";
    static ref TEST_USER_2_EMAIL: &'static str = "testuser2@gmail.com";
    static ref TEST_USER_2_PASSWORD: &'static str = "password";
    static ref TEST_USER_3_NAME: &'static str = "testuser3";
    static ref TEST_USER_3_EMAIL: &'static str = "testuser3@gmail.com";
    static ref TEST_USER_3_PASSWORD: &'static str = "password";
}

pub struct SeedService {
    user_services: DynUsersService,
    category_services: DynCategoriesService,
}

impl SeedService {
    pub fn new(services: Services) -> Self {
        Self {
            user_services: services.users,
            category_services: services.categories,
        }
    }

    pub async fn seed(&self) -> AppResult<()> {
        // assume that if we have an active user in the users table, data has been seeded
        let seed_data_exists = self
            .user_services
            .signin_user(
                SignInUserDto {
                    email: Some(String::from(*TEST_USER_1_EMAIL)),
                    password: Some(String::from(*TEST_USER_1_PASSWORD)),
                },
                Some(String::from("Seed Agent")),
            )
            .await
            .is_ok();

        if seed_data_exists {
            info!("data has already been seeded, bypassing test data setup");
            return Ok(());
        }
        info!("seeding users...");
        let created_user_1 = self
            .create_user(*TEST_USER_1_NAME, *TEST_USER_1_EMAIL, *TEST_USER_1_PASSWORD)
            .await?;

        let created_user_2 = self
            .create_user(*TEST_USER_2_NAME, *TEST_USER_2_EMAIL, *TEST_USER_2_PASSWORD)
            .await?;

        let created_user_3 = self
            .create_user(*TEST_USER_3_NAME, *TEST_USER_3_EMAIL, *TEST_USER_3_PASSWORD)
            .await?;

        info!("users created, seeding categories...");

        let created_users = vec![created_user_1, created_user_2, created_user_3];
        for user in created_users.iter() {
            for index in 1..5 {
                self.category_services
                    .create_category(
                        user.id,
                        CategoryCreateDto {
                            name: Some(format!("{:?} category {:?}", user.name, index)),
                            cat_type: CategoryType::NonEssential,
                        },
                    )
                    .await?;
            }
        }

        info!("seed ran successfully!");
        Ok(())
    }

    async fn create_user(
        &self,
        name: &'static str,
        email: &'static str,
        password: &'static str,
    ) -> AppResult<ResponseUserDto> {
        self.user_services
            .signup_user(SignUpUserDto {
                name: Some(String::from(name)),
                email: Some(String::from(email)),
                password: Some(String::from(password)),
            })
            .await
    }
}
