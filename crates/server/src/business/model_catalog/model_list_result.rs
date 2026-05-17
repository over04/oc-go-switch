use serde::Serialize;

use crate::business::model_catalog::model_list::ModelListResponse;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ModelListResult {
    Ok(ModelListResponse),
    Error { error: String },
}
