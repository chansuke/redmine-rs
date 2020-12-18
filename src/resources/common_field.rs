use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct CommonField {
    pub id: usize,
    pub name: String,
}
