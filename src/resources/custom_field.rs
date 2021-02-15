use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CustomField {
    pub id: i32,
    pub name: String,
    pub value: Option<String>,
}
