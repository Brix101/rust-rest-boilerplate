use std::sync::Arc;
use tracing::info;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    database::expense::repository::{DynExpensesRepository, Expense},
    server::{
        dtos::expense_dto::{ExpenseCreateDto, ExpenseResponseDto, ExpenseUpdateDto},
        error::{AppResult, Error},
    },
};

pub type DynExpensesService = Arc<dyn ExpensesServiceTrait + Send + Sync>;

#[async_trait]
pub trait ExpensesServiceTrait {
    async fn create_expense(&self, request: ExpenseCreateDto) -> AppResult<ExpenseResponseDto>;

    async fn get_expense_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<ExpenseResponseDto>;

    async fn get_expenses(&self, user_id: Uuid) -> AppResult<Vec<ExpenseResponseDto>>;

    async fn updated_expense(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: ExpenseUpdateDto,
    ) -> AppResult<ExpenseResponseDto>;

    async fn delete_expense(&self, user_id: Uuid, id: Uuid) -> AppResult<()>;
}

#[derive(Clone)]
pub struct ExpensesService {
    repository: DynExpensesRepository,
}

impl ExpensesService {
    pub fn new(repository: DynExpensesRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ExpensesServiceTrait for ExpensesService {
    async fn create_expense(&self, request: ExpenseCreateDto) -> AppResult<ExpenseResponseDto> {
        let amount = request.amount.unwrap();
        let description = request.description.unwrap_or(String::from(""));
        let category_id = request.category_id.unwrap();

        let created_expense = self
            .repository
            .create_expense(category_id, amount, description)
            .await?;

        info!("user created expense successfully");

        Ok(created_expense.into_dto())
    }

    async fn get_expense_by_id(&self, id: Uuid, user_id: Uuid) -> AppResult<ExpenseResponseDto> {
        info!("searching for existing Expense {:?}", id);
        let expense = self.repository.get_expense_by_id(id).await?;

        if let Some(existing_expense) = expense {
            // verify the user IDs match on the request and the expense
            let expenses = self.repository.get_expenses(user_id).await?;
            if !expenses
                .iter()
                .any(|user_expense| user_expense.id == existing_expense.id)
            {
                return Err(Error::Forbidden);
            }

            return Ok(existing_expense.into_dto());
        }

        Err(Error::NotFound(String::from("expense was not found")))
    }

    async fn get_expenses(&self, user_id: Uuid) -> AppResult<Vec<ExpenseResponseDto>> {
        let expenses = self.repository.get_expenses(user_id).await?;

        self.map_to_expenses(expenses).await
    }

    async fn updated_expense(
        &self,
        id: Uuid,
        user_id: Uuid,
        request: ExpenseUpdateDto,
    ) -> AppResult<ExpenseResponseDto> {
        let expense_to_update = self.repository.get_expense_by_id(id).await?;

        if let Some(existing_expense) = expense_to_update {
            // verify the user IDs match on the request and the expense
            let expenses = self.repository.get_expenses(user_id).await?;
            if !expenses
                .iter()
                .any(|user_expense| user_expense.id == existing_expense.id)
            {
                return Err(Error::Forbidden);
            }

            let updated_amount = request.amount.unwrap_or(existing_expense.amount);
            let updated_description = request.description.unwrap_or(existing_expense.description);
            let updated_category_id = request.category_id.unwrap_or(existing_expense.category_id);

            let updated_expense = self
                .repository
                .update_expense(id, updated_category_id, updated_amount, updated_description)
                .await?;

            return Ok(updated_expense.into_dto());
        }

        Err(Error::NotFound(String::from("Expense was not found")))
    }

    async fn delete_expense(&self, user_id: Uuid, id: Uuid) -> AppResult<()> {
        let expense = self.repository.get_expense_by_id(id).await?;

        if let Some(existing_expense) = expense {
            // verify the user IDs match on the request and the expense
            let expenses = self.repository.get_expenses(user_id).await?;
            if !expenses
                .iter()
                .any(|user_expense| user_expense.id == existing_expense.id)
            {
                return Err(Error::Forbidden);
            }

            self.repository.delete_expense(existing_expense.id).await?;

            return Ok(());
        }

        Err(Error::NotFound(String::from("Expense was not found")))
    }
}

impl ExpensesService {
    async fn map_to_expenses(&self, expenses: Vec<Expense>) -> AppResult<Vec<ExpenseResponseDto>> {
        info!("found {} Expenses", expenses.len());

        let mut mapped_expenses: Vec<ExpenseResponseDto> = Vec::new();

        if !expenses.is_empty() {
            for expense in expenses {
                mapped_expenses.push(expense.into_dto());
            }
        }

        Ok(mapped_expenses)
    }
}
