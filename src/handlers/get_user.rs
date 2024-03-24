use crate::{models::Employee, storage::get_employee_by_id};
use axum::{extract::{Path}, response::Json};
use uuid::Uuid;

pub async fn get_user(Path(id): Path<Uuid>) -> Json<Employee> {
    let employee = get_employee_by_id(id).await.unwrap();

    Json(employee.expect("cant get user"))
}
