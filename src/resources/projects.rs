use crate::resources::project::ProjectObject;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Projects {
    pub projects: Vec<ProjectObject>,
    // Part of the API response; only the tests read it so far.
    #[allow(dead_code)]
    pub total_count: i32,
}
