use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct CustomField {
    id: i32,
    name: String,
    value: Option<String>,
}
