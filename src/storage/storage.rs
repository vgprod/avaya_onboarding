use uuid::Uuid;

use crate::models::Employee;
use std::error::Error;
use std::fs;
use std::sync:: Mutex;

static FILE_LOCK: Mutex<()> = Mutex::new(()); 

const FILE_PATH: &str = "data.json";

// Function to load employees from JSON file
pub async fn load_employees_from_file() -> Result<Vec<Employee>, Box<dyn std::error::Error>> {
    let _lock = FILE_LOCK.lock().unwrap(); 
    
    let json_data = fs::read_to_string(FILE_PATH)?;
    let employees: Vec<Employee> = serde_json::from_str(&json_data)?;
    Ok(employees)
}

// Function to append a new employee and save employees to JSON file
pub async fn save_employee_to_file(new_employee: &Employee) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut employees = match load_employees_from_file().await {
        Ok(existing_employees) => existing_employees,
        Err(_) => Vec::new(),
    };

    let _lock = FILE_LOCK.lock().unwrap(); 

    employees.push(new_employee.clone());

    let json_data = serde_json::to_string_pretty(&employees)?;

    fs::write(FILE_PATH, json_data)?;

    Ok(())
}

pub async fn get_employee_by_id(id: Uuid) -> Result<Option<Employee>, Box<dyn Error>> {
    let employees = load_employees_from_file().await.unwrap_or_default();
    Ok(employees.into_iter().find(|e| e.id == Some(id.to_string())))
}
