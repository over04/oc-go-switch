use serde::Serialize;

use crate::business::model_catalog::model_list_result::ModelListResult;

#[derive(Debug, Serialize)]
pub struct MergedModelsResponse {
    pub openai: ModelListResult,
    pub claude: ModelListResult,
}
