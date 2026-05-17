use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SetActiveKeyRequest {
    pub key_id: String,
}

#[derive(Debug, Serialize)]
pub struct ActiveKeyActionResponse {
    pub status: &'static str,
}
