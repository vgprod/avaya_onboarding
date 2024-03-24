use crate::{models::Employee, storage::load_employees_from_file};
use axum::response::Json;

//Retrieves a collection of all non boraded applicants in JSON format.
//ex. http://127.0.0.1:8100/applicants

pub async fn get_applicants() -> Json<Vec<String>> {
    let employees = load_employees_from_file().await;
    match employees {
        Ok(users) => {
            let non_onboarded_employees  = users
                .iter()
                .filter(|employee| !employee.onboarded.expect("Error"))
                .map(|non_onb| non_onb.id.clone().expect("Error"))
                .collect();
            //let vecid = non_onboarded_employees.iter().map(|non_onb| non_onb.id.clone().expect("Error")).collect();
            Json(non_onboarded_employees)
        }
        Err(err) => {
            // Handle error here if needed
            panic!("Error loading employees: {:?}", err);
        }
    }
}
