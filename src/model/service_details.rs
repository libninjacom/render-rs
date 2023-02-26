
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDetails {
    #[serde(rename = "buildCommand")]
    pub build_command: String,
    #[serde(rename = "parentServer")]
    pub parent_server: serde_json::Value,
    #[serde(rename = "publishPath")]
    pub publish_path: String,
    #[serde(rename = "pullRequestPreviewsEnabled")]
    pub pull_request_previews_enabled: String,
    pub url: String,
}
impl std::fmt::Display for ServiceDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}