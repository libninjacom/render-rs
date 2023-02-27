
use serde::{Serialize, Deserialize};
use super::{EnvVar, SecretFile, ServiceLink};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvGroup {
    #[serde(rename = "envVars")]
    pub env_vars: Vec<EnvVar>,
    pub id: String,
    pub name: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "secretFiles")]
    pub secret_files: Vec<SecretFile>,
    #[serde(rename = "serviceLinks")]
    pub service_links: Vec<ServiceLink>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for EnvGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}