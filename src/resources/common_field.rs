use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommonField {
    id: usize,
    name: String,
}
