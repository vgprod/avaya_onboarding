use crate::{models::Employee, storage::load_employees_from_file};
use axum::response::Json;


pub async fn get_applicants() -> Json<Vec<Employee>> {
    let employees = load_employees_from_file();
    Json(employees.unwrap())
}