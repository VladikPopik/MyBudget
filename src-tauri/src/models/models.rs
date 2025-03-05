use crate::schema::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, Debug, Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Users {
    pub user_login: String,
    pub user_name: String,
    pub user_email: String,
    pub tg_login: String,
    pub user_password: String,
}

#[derive(Queryable, Selectable, Clone, Debug, Insertable)]
#[diesel(belongs_to(Users))]
#[diesel(table_name = schema::budgets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Budgets {
    pub budget_id: String,
    pub budget_name: String,
    pub budget_type: String,
    pub ts_start: i32,
    pub duration: i32,
    pub budget_limit: f32,
    pub budget_pred: Option<f32>,
}
