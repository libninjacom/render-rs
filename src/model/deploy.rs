
use serde::{Serialize, Deserialize};
use super::Commit;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deploy {
    pub commit: Commit,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "finishedAt")]
    pub finished_at: String,
    pub id: String,
    pub status: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}
impl std::fmt::Display for Deploy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}