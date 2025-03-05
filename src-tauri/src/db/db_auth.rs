use super::db_instance::DBInstance;
use crate::models::models::Users;
use diesel::prelude::*;

pub async fn read_user(login: String) -> Users {
    use crate::schema::schema::users::dsl::*;
    let mut db_instance = DBInstance::new();

    let connection = &mut db_instance.connection;

    let user_select = users
        .select(Users::as_select())
        .filter(user_login.eq(login))
        .limit(1)
        .load(connection)
        .expect("Error handling read users from db");
    user_select.first().unwrap().clone()
}

pub async fn create_user(login: String, name: String, email: String, tg: String, password: String) {
    use crate::schema::schema::users::dsl::*;

    let mut db_instance = DBInstance::new();
    let connection = &mut db_instance.connection;

    let new_user = Users {
        user_login: login,
        user_name: name,
        user_email: email,
        tg_login: tg,
        user_password: password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .returning(Users::as_returning())
        .get_result(connection)
        .expect("Error saing new user");
}
