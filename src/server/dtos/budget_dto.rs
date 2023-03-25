use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::database::budget::repository::{Budget, PlanType};

impl Budget {
    pub fn into_dto(self) -> BudgetResponseDto {
        BudgetResponseDto {
            id: self.id,
            category_id: self.category_id,
            amount: Some(self.amount),
            description: Some(self.description),
            plan: self.plan,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct BudgetResponseDto {
    pub id: Uuid,
    pub category_id: Uuid,
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub plan: PlanType,
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct BudgetCreateDto {
    #[validate(required)]
    pub category_id: Option<Uuid>,
    #[validate(required, range(min = 0.00))]
    pub amount: Option<f64>,
    pub description: Option<String>,
    #[validate(required)]
    pub plan: Option<PlanType>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BudgetUpdateDto {
    pub category_id: Option<Uuid>,
    pub amount: Option<f64>,
    pub description: Option<String>,
    pub plan: Option<PlanType>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BudgetQuery {
    pub budget_id: Option<Uuid>,
}
