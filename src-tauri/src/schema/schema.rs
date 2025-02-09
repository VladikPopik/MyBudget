diesel::table! {
    users(user_login) {
        user_login -> Varchar,
        user_name -> Varchar,
        user_email -> Varchar,
        tg_login -> Varchar
    }
}