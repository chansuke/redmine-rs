use anyhow::Result;

use crate::resources::trackers::Trackers;
use crate::RmError;

pub(crate) async fn get_trackers(endpoint: &str) -> Result<Trackers, RmError> {
    let response = reqwest::get(endpoint).await?.text().await?;
    let result: Trackers = serde_json::from_str(&response)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::resources::trackers::TrackerObject;

    use super::*;

    #[tokio::test]
    async fn test_get_trackers() {
        let endpoint = "https://www.redmine.org/trackers.json";
        let result = get_trackers(endpoint).await.unwrap();
        let trackers = r#"[
          {
              "id": 1,
              "name": "Defect"
          },
          {
              "id": 2,
              "name": "Feature"
          },
          {
              "id": 3,
              "name": "Patch"
          }
        ]"#;
        let tracker_objects: Vec<TrackerObject> = serde_json::from_str(trackers).unwrap();
        assert_eq!(result.trackers, tracker_objects);
    }
}
