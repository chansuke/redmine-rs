use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CustomField {
    id: i32,
    name: String,
    value: Option<String>,
}
