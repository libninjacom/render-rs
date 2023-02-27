
use serde::{Serialize, Deserialize};
use super::EnvGroup;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvGroupCursor {
    pub cursor: String,
    #[serde(rename = "envGroup")]
    pub env_group: EnvGroup,
}
impl std::fmt::Display for EnvGroupCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}