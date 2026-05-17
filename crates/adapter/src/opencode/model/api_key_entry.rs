use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEntry {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
}
