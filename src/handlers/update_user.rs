use crate::{
    models::Employee,
    storage::{
        get_employee_by_id, get_employee_by_password, load_employees_from_file,
        save_employee_to_file,
    },
};
use axum::{extract::Path, http::StatusCode, response::Json};
use rand::seq::SliceRandom;
use uuid::Uuid;

// The update_user retrieves a particular user based on the provided ID in the Path, for example,
// http://127.0.0.1:8100/update/6755c364-27db-4b06-b645-d89127304e5a, and assigns a new handle and password to it.
// This process ensures uniqueness by adding a number to the handle and regenerating the password if necessary.

pub async fn update_user(Path(id): Path<Uuid>) -> Result<Json<Employee>, StatusCode> {
    let employees = match load_employees_from_file().await {
        Ok(employees) => employees,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut employee = match get_employee_by_id(id).await {
        Some(employee) => employee,
        None => return Err(StatusCode::NOT_FOUND),
    };

    employee.user_handle =
        Some(generate_user_handle(&employee.first_name, &employee.last_name, &employees).await);

    loop {
        let gen_password = Some(generate_password().await);

        if is_password_unique(
            gen_password
                .as_ref()
                .expect("Password not unique. Regenerating..."),
        )
        .await
        {
            employee.password = gen_password;
            break;
        }
    }

    employee.onboarded = Some(true);

    if let Err(_) = save_employee_to_file(&employee).await {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(employee))
}

async fn generate_user_handle(first_name: &str, last_name: &str, employees: &[Employee]) -> String {
    let mut handle = format!("{}.{}", first_name.to_lowercase(), last_name.to_lowercase());
    handle = handle.replace(" ", "");

    if employees
        .iter()
        .any(|e| e.user_handle == Some(handle.clone()))
    {
        let mut counter = 1;
        loop {
            let unique_handle = format!("{}{}", handle, counter);
            if !employees
                .iter()
                .any(|e| e.user_handle == Some(unique_handle.clone()))
            {
                return unique_handle;
            }
            counter += 1;
        }
    }

    handle
}

async fn generate_password() -> String {
    let mut password;

    // Generate random characters for the password
    let lowercase = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();
    let uppercase = (b'A'..=b'Z').map(char::from).collect::<Vec<_>>();
    let numbers = (b'0'..=b'9').map(char::from).collect::<Vec<_>>();
    let special_chars = "!@#$%^&*()-_=+[]{}|;:,.<>?".chars().collect::<Vec<_>>();

    let mut rng = rand::thread_rng();

    loop {
        password = String::new();

        let lowercase_char = *lowercase.choose(&mut rng).unwrap();
        let uppercase_char = *uppercase.choose(&mut rng).unwrap();
        let number_char = *numbers.choose(&mut rng).unwrap();
        let special_char = *special_chars.choose(&mut rng).unwrap();

        // First character
        let first_char = [lowercase_char, number_char, special_char];
        let random_char = *first_char.choose(&mut rng).unwrap();
        password.push(random_char);

        // Middle characters
        for _ in 0..7 {
            let chars = [lowercase_char, uppercase_char, number_char, special_char];
            let remaining_chars = lowercase
                .iter()
                .chain(uppercase.iter())
                .chain(numbers.iter())
                .chain(special_chars.iter())
                .filter(|c| !chars.contains(c))
                .collect::<Vec<_>>();

            let random_char = *remaining_chars.choose(&mut rng).unwrap();
            password.push(*random_char);
        }

        // Last character
        let last_char = [lowercase_char, number_char, uppercase_char];
        let random_char = *last_char.choose(&mut rng).unwrap();
        password.push(random_char);

        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_number = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password
            .chars()
            .any(|c| "!@#$%^&*()-_=+[]{}|;:,.<>?".contains(c));

        // Check if password length is 9
        if password.len() == 9 && has_uppercase && has_lowercase && has_number && has_special {
            break;
        }
    }

    password
}

async fn is_password_unique(password: &String) -> bool {
    match get_employee_by_password(password).await {
        Some(_) => false,
        None => true,
    }
}
