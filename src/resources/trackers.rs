use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct TrackerObject {
    pub id: i32,
    pub name: String,
    pub dafault_status: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Trackers {
    pub trackers: Vec<TrackerObject>,
}
