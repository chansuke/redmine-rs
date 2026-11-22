use crate::resources::common_field::CommonField;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct MembershipObject {
    pub id: i32,
    pub project: CommonField,
    pub user: CommonField,
    pub roles: Vec<CommonField>,
}

#[derive(Deserialize, Debug)]
pub struct Memberships {
    pub memberships: Vec<MembershipObject>,
    // Part of the API response; only the tests read it so far.
    #[allow(dead_code)]
    pub total_count: i32,
}
