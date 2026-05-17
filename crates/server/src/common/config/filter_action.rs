use serde::{Deserialize, Serialize};

/// 图片过滤行为。
///
/// 该枚举是配置文件中的闭集，代理层直接按枚举处理，避免字符串分发。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAction {
    /// 保留图片，不改变协议 payload。
    #[serde(rename = "pass_through")]
    PassThrough,
    /// 移除图片 block，只保留其余文本和结构。
    Remove,
    /// 将图片 block 替换为配置文本。
    Replace,
}
