use uuid::Uuid;

use crate::models::Employee;
use std::fs;
use std::sync::Mutex;

static FILE_LOCK: Mutex<()> = Mutex::new(());

const FILE_PATH: &str = "data.json";

// Function to load employees from JSON file
pub async fn load_employees_from_file() -> Result<Vec<Employee>, Box<dyn std::error::Error>> {
    let _lock = FILE_LOCK.lock().unwrap();

    let json_data = fs::read_to_string(FILE_PATH)?;
    let employees: Vec<Employee> = serde_json::from_str(&json_data)?;
    Ok(employees)
}

// Function to append a new employee and save changes of employees to JSON file
pub async fn save_employee_to_file(
    employee_data: &Employee,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut employees = load_employees_from_file().await?;
    let _lock = FILE_LOCK.lock().unwrap();

    if let Some(id) = &employee_data.id {
        if let Some(index) = employees.iter().position(|e| e.id == Some(id.to_string())) {
            employees[index] = employee_data.clone();
        } else {
            employees.push(employee_data.clone());
        }
    } else {
        employees.push(employee_data.clone());
    }

    let json_data = serde_json::to_string_pretty(&employees)?;
    fs::write(FILE_PATH, json_data)?;

    Ok(())
}

pub async fn get_employee_by_id(id: Uuid) -> Option<Employee> {
    let employees = load_employees_from_file().await.unwrap_or_default();
    employees.into_iter().find(|e| e.id == Some(id.to_string()))
}

pub async fn get_employee_by_password(password: &str) -> Option<Employee> {
    let employees = load_employees_from_file().await.unwrap_or_default();
    employees
        .into_iter()
        .find(|e| e.password.as_deref() == Some(password))
}

pub async fn get_employee_by_user_handle(user_handle: &str) -> Option<Employee> {
    let employees = load_employees_from_file().await.unwrap_or_default();
    employees
        .into_iter()
        .find(|e| e.user_handle.as_deref() == Some(user_handle))
}

pub async fn authenticate(username: &str, password: &str) -> Option<bool> {
    let employees = load_employees_from_file().await.unwrap_or_default();
    //filter to see if there is user with that username and pass
    let user_found = employees.into_iter().any(|e| {
        e.user_handle.as_deref() == Some(username) && e.password.as_deref() == Some(password)
    });
    Some(user_found)
}
