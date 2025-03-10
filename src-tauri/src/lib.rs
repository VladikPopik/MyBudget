pub mod auth;
pub mod budget;
pub mod db;
pub mod models;
pub mod schema;
use crate::auth::login::login;
use crate::auth::register::register;
use crate::auth::token::verify_token;
use crate::budget::budget_handlers::{create_budget_handler, execute_read_budgets_by_user};
use budget::budget::SharedBudgets;
use std::sync::Mutex;
use tauri::{Builder, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(SharedBudgets::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            login,
            register,
            verify_token,
            create_budget_handler,
            execute_read_budgets_by_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
