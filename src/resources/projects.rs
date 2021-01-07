use crate::resources::project::ProjectObject;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Projects {
    pub projects: Vec<ProjectObject>,
    pub total_count: i32,
}
