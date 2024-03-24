pub mod storage;

pub use storage::{
    load_employees_from_file, save_employee_to_file, get_employee_by_id, get_employee_by_password, get_employee_by_user_handle, authenticate
};
