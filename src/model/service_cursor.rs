
use serde::{Serialize, Deserialize};
use super::Service;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCursor {
    pub cursor: String,
    pub service: Service,
}
impl std::fmt::Display for ServiceCursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}