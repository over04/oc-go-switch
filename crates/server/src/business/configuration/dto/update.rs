use serde::Deserialize;

use crate::business::configuration::dto::runtime_update::ConfigurationRuntimeUpdateReqDto;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConfigurationUpdateReqDto {
    pub runtime: Option<ConfigurationRuntimeUpdateReqDto>,
}
