
use serde::{Serialize, Deserialize};
use super::ServiceDetails;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "autoDeploy")]
    pub auto_deploy: String,
    pub branch: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "notifyOnFail")]
    pub notify_on_fail: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    pub repo: String,
    #[serde(rename = "serviceDetails")]
    pub service_details: ServiceDetails,
    pub slug: String,
    pub suspended: String,
    pub suspenders: Vec<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}