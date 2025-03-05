use super::db_instance::DBInstance;
use crate::models::models::Budgets;
use crate::schema::schema::budgets::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;

pub async fn create_budget(
    b_name: String,
    b_type: String,
    b_ts_start: i32,
    b_duration: i32,
    b_limit: f32,
    b_pred: f32,
) {
    let connection = &mut DBInstance::connection();

    let new_budget = Budgets {
        budget_id: Uuid::new_v4().to_string(),
        budget_name: b_name,
        budget_type: b_type,
        ts_start: b_ts_start,
        duration: b_duration,
        budget_limit: b_limit,
        budget_pred: Some(b_pred),
    };

    diesel::insert_into(budgets)
        .values(&new_budget)
        .returning(Budgets::as_returning())
        .get_result(connection)
        .expect("Error creating budgets");
}

pub async fn read_budgets_by_name(b_name: String) -> Vec<Budgets> {
    let connection = &mut DBInstance::connection();

    let budget_select = budgets
        .select(Budgets::as_select())
        .filter(budget_name.eq(b_name))
        .load(connection)
        .expect("Error while reading budgets");

    budget_select
}

pub async fn read_budgets_by_user(user: String) -> Vec<Budgets> {
    let connection = &mut DBInstance::connection();

    let budget_select = budgets
        .select(Budgets::as_select())
        .filter(user_login.eq(user))
        .load(connection)
        .expect("Error while reading budgets");

    budget_select
}

pub async fn read_budgets() -> Vec<Budgets> {
    let connection = &mut DBInstance::connection();

    let budget_select = budgets
        .select(Budgets::as_select())
        .load(connection)
        .expect("Error while reading budgets");

    budget_select
}

pub async fn read_budget_by_uuid(uuid: String) -> Budgets {
    let connection = &mut DBInstance::connection();

    let budget_select = budgets
        .select(Budgets::as_select())
        .filter(budget_id.eq(uuid))
        .load(connection)
        .expect("Error while reading budgets");

    budget_select.first().unwrap().clone()
}

pub async fn update_budget(
    uuid: String,
    b_name: String,
    b_type: String,
    b_ts_start: i32,
    b_duration: i32,
    b_limit: f32,
    b_pred: f32,
) {
    let connection = &mut DBInstance::connection();

    diesel::update(budgets)
        .set((
            ts_start.eq(b_ts_start),
            duration.eq(b_duration),
            budget_limit.eq(b_limit),
            budget_pred.eq(b_pred),
            budget_name.eq(b_name),
            budget_type.eq(b_type),
        ))
        .filter(budget_id.eq(uuid))
        .execute(connection)
        .expect("Error while updating budget");
}

pub async fn delete_budget(uuid: String) {
    let connection = &mut DBInstance::connection();

    diesel::delete(budgets)
        .filter(budget_id.eq(uuid))
        .execute(connection)
        .expect("Error while deleting budget");
}
