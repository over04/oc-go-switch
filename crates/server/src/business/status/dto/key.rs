use serde::Serialize;

use crate::business::workspace::key::KeyStatus;

#[derive(Debug, Serialize)]
pub struct KeyStatusEntry {
    pub id: String,
    pub masked: String,
    pub status: KeyStatus,
}
