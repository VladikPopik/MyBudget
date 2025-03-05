diesel::table! {
    users(user_login) {
        user_login -> Varchar,
        user_name -> Varchar,
        user_email -> Varchar,
        tg_login -> Varchar,
        user_password -> VarChar
    }
}

diesel::table! {
    budgets(budget_id) {
        budget_id -> Varchar,
        budget_name -> Varchar,
        budget_type -> Varchar,
        user_login -> Varchar,
        ts_start -> Integer,
        duration -> Integer,
        budget_limit -> Float,
        budget_pred -> Nullable<Float>,
    }
}

diesel::joinable!(budgets -> users(user_login));

diesel::allow_tables_to_appear_in_same_query!(budgets, users);
