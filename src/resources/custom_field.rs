use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct CustomField {
    pub id: i32,
    pub name: String,
    pub value: Option<String>,
}
