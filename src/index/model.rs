use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiInfo {
    pub instance_id: String,
    pub status: String,
    pub timestamp: u64,
}
