use chrono::Utc;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone,Debug)]
pub struct Employee {
    pub id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub user_handle: Option<String>,
    pub password: Option<String>,
    pub age: u8,
    pub diplomas: Vec<String>,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub onboarded:Option<bool>,
}

impl Employee {
    pub fn new(id: Option<String>, first_name: String, last_name: String, user_handle: Option<String>, password: Option<String>, age: u8, diplomas: Vec<String>, created_at: Option<chrono::DateTime<Utc>>,onboarded: Option<bool>) -> Self {
        Self { id, first_name, last_name, user_handle, password, age, diplomas, created_at,onboarded }
    }
}
