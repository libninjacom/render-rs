
use serde::{Serialize, Deserialize};
use super::Owner;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnerCursor {
    pub cursor: String,
    pub owner: Owner,
}
impl std::fmt::Display for OwnerCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}