use crate::auth::hash_pass::hash_pass;
use crate::db::db_auth::create_user;

#[tauri::command]
pub async fn register(login: String, name: String, email: String, tg: String, password: String) {
    let password = hash_pass(login.clone(), password);

    let _user = create_user(login, name, email, tg, password).await;
}
