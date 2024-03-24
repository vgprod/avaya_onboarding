use crate::{models::Employee, storage::save_employee_to_file};
use uuid::Uuid;
use chrono::Utc;
use axum::{
    http::StatusCode,
    response::Json,
};

pub async fn create_user() -> Result<Json<Employee>, StatusCode> {
    let new_employee = Employee::new(
        Uuid::new_v4().to_string(), 
        "John".to_string(),
        "Doe".to_string(),
        "userhandle".to_string(),
        "pass".to_string(),
        30,
        vec![],
        Utc::now(),
    );
    save_employee_to_file(&new_employee).unwrap();
    

   Ok(Json(new_employee))
}



