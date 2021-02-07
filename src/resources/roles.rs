use super::common_field::CommonField;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Roles {
    pub roles: Vec<CommonField>,
}
