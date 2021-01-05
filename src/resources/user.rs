use crate::resources::custom_field::CustomField;
use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct UserObject {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub mail: String,
    pub created_on: DateTime<Local>,
    pub last_login_on: DateTime<Local>,
    pub custom_fields: Vec<CustomField>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct User {
    pub user: UserObject,
}
