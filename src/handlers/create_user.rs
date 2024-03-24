use crate::{models::Employee, storage::save_employee_to_file};
use axum::{extract, http::StatusCode, response};
use chrono::Utc;
use uuid::Uuid;

// Receives data in JSON format and generates a user application.
// The application is created only if person possesses a diploma or is aged 18 or older.
// For instance, it can be accessed via http://127.0.0.1:8100/sign_up with the following body:

// json

// {
//    "first_name": "TEST",
//    "last_name": "TEST",
//    "age": 33,
//    "diplomas": ["TEST"]
// }

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

    let new_employee = Employee::new(
        Some(Uuid::new_v4().to_string()),
        first_name,
        last_name,
        None,
        None,
        input.age.clone(),
        input.diplomas.clone(),
        Some(Utc::now()),
        Some(false)
    );
    save_employee_to_file(&new_employee).await.unwrap();

    Ok(response::Json(new_employee))
}

