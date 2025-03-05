pub mod auth;
pub mod db;
pub mod models;
pub mod schema;
use crate::auth::login::login;
use crate::auth::register::register;
use crate::auth::token::verify_token;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![login, register, verify_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
