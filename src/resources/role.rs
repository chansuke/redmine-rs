use super::common_field::CommonField;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Role {
    pub role: CommonField,
}
