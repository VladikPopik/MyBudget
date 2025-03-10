use uuid::Uuid;

use crate::models::models::Budgets;

enum BudgetType {
    FOOD = 1,
    SMOKE = 2,
    ARENDA = 3,
    TODO,
}

impl BudgetType {
    fn as_string(&self) -> String {
        let budget_type_map;
        match self {
            BudgetType::FOOD => budget_type_map = "food".to_string(),
            BudgetType::SMOKE => budget_type_map = "smoke".to_string(),
            BudgetType::ARENDA => budget_type_map = "arenda".to_string(),
            BudgetType::TODO => todo!("NOT IMPLEMENTED"),
        }
        budget_type_map
    }
}

#[derive(Clone, Debug)]
pub struct Budget {
    pub budget_id: String,
    pub budget_name: String,
    pub budget_type: String,
    pub ts_start: i32,
    pub duration: i32,
    pub budget_limit: f32,
    pub budget_pred: Option<f32>,
}

impl Budget {
    pub fn new(
        budget_name: String,
        budget_type: BudgetType,
        ts_start: i32,
        duration: i32,
        budget_limit: f32,
        budget_pred: Option<f32>,
    ) -> Self {
        Budget {
            budget_id: Uuid::new_v4().to_string(),
            budget_name,
            budget_type: budget_type.as_string(),
            ts_start,
            duration,
            budget_limit,
            budget_pred,
        }
    }

    pub fn new_from_raw(budget: Budgets) -> Self {
        Budget {
            budget_id: budget.budget_id,
            budget_name: budget.budget_name,
            budget_type: budget.budget_type,
            ts_start: budget.ts_start,
            duration: budget.duration,
            budget_limit: budget.budget_limit,
            budget_pred: budget.budget_pred,
        }
    }
}

#[derive(Default)]
pub(crate) struct SharedBudgets {
    pub budgets: Vec<Budget>,
}
