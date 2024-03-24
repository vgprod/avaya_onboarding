use crate::{models::Employee, storage::load_employees_from_file};
use axum::response::Json;


pub async fn get_applicants() -> Json<Vec<Employee>> {
    let employees = load_employees_from_file();
    match employees {
        Ok(users) => {
            let non_onboarded_employees: Vec<Employee> = users
                .iter()
                .filter(|employee| !employee.onboarded.unwrap_or(false))
                .cloned()
                .collect();
            Json(non_onboarded_employees)
        }
        Err(err) => {
            // Handle error here if needed
            panic!("Error loading employees: {:?}", err);
        }
    }
}