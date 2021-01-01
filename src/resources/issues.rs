use crate::resources::issue::IssueObject;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issues {
    issues: Vec<IssueObject>,
}
