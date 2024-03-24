use crate::{models::Employee, storage::get_employee_by_user_handle, storage::authenticate};
use axum::{extract::Path, response::Json, http::StatusCode};
use axum_auth::AuthBasic;

pub async fn get_user_details(AuthBasic((username, password)): AuthBasic, Path(user_handle): Path<String>) -> Result<Json<Employee>, StatusCode> {
    // Authenticate user
    if let Some(true) = authenticate(&username, &password.expect("Authentication error.")).await {
        // Get employee details if authentication succeeds
        let employee = get_employee_by_user_handle(&user_handle).await.expect("Can't get employee by that handle.");
        return Ok(Json(employee));
    }
    // If authentication fails, return 401 Unauthorized
    return Err(StatusCode::UNAUTHORIZED);
}
