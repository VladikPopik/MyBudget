use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Users {
    pub user_loign: String,
    pub user_name: String,
    pub user_email: String,
    pub tg_login: bool,
}