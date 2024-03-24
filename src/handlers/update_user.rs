use std::vec;

use axum::{extract::{self, Query}, http::StatusCode, response};
use chrono::Utc;
use uuid::Uuid;

use crate::{models::Employee, storage::{save_employee_to_file, get_employee_by_id}};

pub async fn update_user
    (Query(id): Query<Uuid>,
) -> Result<response::Json<Employee>, StatusCode> {
    get_employee_by_id(id).await.unwrap();
    let first_name= "test".to_owned();
      let  last_name = "test".to_owned();
    let user_handle = generate_handle(&first_name, &last_name).await;
    let password = generate_password().await;

    let update_employee = Employee::new(
        Some(Uuid::new_v4().to_string()),
        first_name,
        last_name,
        Some(user_handle),
        Some(password),
        20,
        vec!["".to_owned()],
        Some(Utc::now()),
        Some(true)
    );
    //save_employee_to_file(&new_employee).await.unwrap();

    Ok(response::Json(update_employee))
}

async fn generate_handle(first: &str, last: &str) -> String {
    let handle = format!("{}.{}", first, last);
    handle
}

async fn generate_password() -> String {
    "password".to_string()
}
