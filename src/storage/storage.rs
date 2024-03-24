use crate::models::Employee;
use std::fs;
use std::sync:: Mutex;

static FILE_LOCK: Mutex<()> = Mutex::new(()); 

const FILE_PATH: &str = "data.json";

// Function to load employees from JSON file
pub fn load_employees_from_file() -> Result<Vec<Employee>, Box<dyn std::error::Error>> {
    let _lock = FILE_LOCK.lock().unwrap(); 
    
    let json_data = fs::read_to_string(FILE_PATH)?;
    let employees: Vec<Employee> = serde_json::from_str(&json_data)?;
    Ok(employees)
}

// Function to append a new employee and save employees to JSON file
pub fn save_employee_to_file(new_employee: &Employee) -> Result<(), Box<dyn std::error::Error>> {
    let _lock = FILE_LOCK.lock().unwrap(); 
    
    let mut employees = match load_employees_from_file() {
        Ok(existing_employees) => existing_employees,
        Err(_) => Vec::new(),
    };

    employees.push(new_employee.clone());

    let json_data = serde_json::to_string_pretty(&employees)?;

    fs::write(FILE_PATH, json_data)?;

    Ok(())
}
