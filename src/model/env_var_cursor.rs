
use serde::{Serialize, Deserialize};
use super::EnvVar;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVarCursor {
    pub cursor: String,
    #[serde(rename = "envVar")]
    pub env_var: EnvVar,
}
impl std::fmt::Display for EnvVarCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}