
use serde::{Serialize, Deserialize};
use super::Deploy;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeployCursor {
    pub cursor: String,
    pub deploy: Deploy,
}
impl std::fmt::Display for DeployCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}