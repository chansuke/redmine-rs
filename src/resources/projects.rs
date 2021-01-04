use crate::resources::project::ProjectObject;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Projects {
    projects: Vec<ProjectObject>,
}
