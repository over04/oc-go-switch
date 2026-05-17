use serde::{Deserialize, Serialize};

use crate::business::model_catalog::model_info::ModelInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelListResponse {
    pub object: String,
    pub data: Vec<ModelInfo>,
}
