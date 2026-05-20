use serde::Deserialize;

use crate::common::config::{go::GoConfig, image_filter::ImageFilterConfig};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationRuntimeUpdateReqDto {
    pub refresh_interval_secs: Option<u64>,
    pub max_retries: Option<usize>,
    pub go: Option<GoConfig>,
    pub image_filter: Option<ImageFilterConfig>,
    pub api_token: Option<String>,
}
