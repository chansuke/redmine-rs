use crate::resources::issue::IssueObject;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issues {
    pub issues: Vec<IssueObject>,
}
