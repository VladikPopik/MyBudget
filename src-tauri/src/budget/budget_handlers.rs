use crate::budget::budget::Budget;
use crate::db::db_budget::{create_budget, read_budgets_by_user};
use crate::SharedBudgets;
use std::sync::Mutex;
use tauri::State;
struct BudgetHandler;

impl BudgetHandler {
    async fn read_budgets_by_user(
        mut budgets: Vec<Budget>,
        user: String,
    ) -> Result<Vec<Budget>, ()> {
        let raw_budgets = read_budgets_by_user(user).await;
        for budget in raw_budgets {
            budgets.push(Budget::new_from_raw(budget));
        }

        Ok(budgets)
    }
}

#[tauri::command]
pub async fn create_budget_handler(
    b_name: String,
    b_type: String,
    b_ts_start: i32,
    b_duration: i32,
    b_limit: f32,
) {
    create_budget(b_name, b_type, b_ts_start, b_duration, b_limit, 0.0).await;
}

#[tauri::command]
pub async fn execute_read_budgets_by_user(
    state: State<'_, Mutex<SharedBudgets>>,
    user: String,
) -> Result<Vec<Budget>, ()> {
    let mut curr_state = state.lock().unwrap();
    curr_state.budgets =
        BudgetHandler::read_budgets_by_user(curr_state.budgets.clone(), user).await?;
    Ok(curr_state.budgets.clone())
}
