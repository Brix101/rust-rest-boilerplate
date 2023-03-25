use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::database::category::repository::{Category, CategoryType};

impl Category {
    pub fn into_dto(self) -> CategoryResponseDto {
        CategoryResponseDto {
            id: self.id,
            name: Some(self.name),
            cat_type: self.cat_type,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CategoryResponseDto {
    pub id: Uuid,
    pub name: Option<String>,
    pub cat_type: CategoryType,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct CategoryCreateDto {
    #[validate(required, length(min = 1))]
    pub name: Option<String>,
    pub cat_type: CategoryType,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CategoryUpdateDto {
    pub name: Option<String>,
    pub cat_type: Option<CategoryType>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CategoryQuery {
    pub category_id: Option<Uuid>,
}

impl CategoryCreateDto {
    pub fn new_stub() -> Self {
        Self {
            name: Some(String::from("stub category")),
            cat_type: CategoryType::NonEssential,
        }
    }
}
