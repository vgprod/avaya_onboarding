use crate::{models::Employee, storage::save_employee_to_file};
use axum::{extract, http::StatusCode, response};
use chrono::Utc;
use uuid::Uuid;

pub async fn create_user(
    input: extract::Json<Employee>,
) -> Result<response::Json<Employee>, StatusCode> {
    // Validate age
    if input.age < 18 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Validate diplomas
    if input.diplomas.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let first_name = input.first_name.clone();
    let last_name = input.last_name.clone();
    let user_handle = generate_handle(&first_name, &last_name).await;
    let password = generate_password().await;

    let new_employee = Employee::new(
        Some(Uuid::new_v4().to_string()),
        first_name,
        last_name,
        Some(user_handle),
        Some(password),
        input.age.clone(),
        input.diplomas.clone(),
        Some(Utc::now()),
        Some(false)
    );
    save_employee_to_file(&new_employee).await.unwrap();

    Ok(response::Json(new_employee))
}

async fn generate_handle(first: &str, last: &str) -> String {
    let handle = format!("{}.{}", first, last);
    handle
}

async fn generate_password() -> String {
    "password".to_string()
}
