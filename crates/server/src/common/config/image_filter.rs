use serde::{Deserialize, Serialize};

use crate::common::config::image_filter_model::ImageFilterModel;

/// 图片过滤配置集合。
///
/// 过滤以模型为粒度配置，空列表表示全部模型透传图片。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ImageFilterConfig {
    /// 按模型配置的图片处理规则。
    #[serde(default)]
    pub models: Vec<ImageFilterModel>,
}
