use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct CommonField {
    id: usize,
    name: String,
}
