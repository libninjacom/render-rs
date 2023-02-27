
use serde::{Serialize, Deserialize};
use super::{EnvVar, SecretFile, ServiceLink};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvGroup {
    #[serde(rename = "envVars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_vars: Option<Vec<EnvVar>>,
    pub id: String,
    pub name: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "secretFiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_files: Option<Vec<SecretFile>>,
    #[serde(rename = "serviceLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_links: Option<Vec<ServiceLink>>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
impl std::fmt::Display for EnvGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}