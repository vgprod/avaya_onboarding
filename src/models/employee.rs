use chrono::Utc;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Employee {
    id: String,
    first_name: String,
    last_name: String,
    user_handle: String,
    password: String,
    age: u8,
    diplomas: Vec<String>,
    created_at: chrono::DateTime<Utc>,
}

impl Employee {
    pub fn new(id: String, first_name: String, last_name: String, user_handle: String, password: String, age: u8, diplomas: Vec<String>, created_at: chrono::DateTime<Utc>) -> Self {
        Self { id, first_name, last_name, user_handle, password, age, diplomas, created_at }
    }
}
