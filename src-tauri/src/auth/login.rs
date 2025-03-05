use crate::db::db_auth::read_user;

#[tauri::command]
pub async fn login(user_login: String, password: String) -> bool {
    let user = read_user(user_login).await;
    if user.user_password == password {
        return true;
    };
    false
}
