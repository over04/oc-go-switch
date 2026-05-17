use serde::{Deserialize, Serialize};

use crate::common::config::filter_action::FilterAction;

/// 单个模型的图片处理规则。
///
/// `model` 使用精确匹配，避免通配规则误伤相近模型名。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFilterModel {
    /// 模型 ID（精确匹配）。
    pub model: String,
    /// 对该模型的图片处理方式。
    pub action: FilterAction,
    /// 替换文本，仅在 `FilterAction::Replace` 时读取。
    #[serde(default)]
    pub replacement: Option<String>,
}
