use crate::redmine::issue::Issue;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Issues {
    issues: Vec<Issue>,
}
