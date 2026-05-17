use serde::{Deserialize, Serialize};

/// 图片过滤行为。
///
/// 该枚举是配置文件中的闭集，代理层直接按枚举处理，避免字符串分发。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum FilterAction {
    /// 保留图片，不改变协议 payload。
    PassThrough,
    /// 移除图片 block，只保留其余文本和结构。
    Remove,
    /// 将图片 block 替换为配置文本。
    Replace {
        /// 用于替换图片 block 的文本。
        replacement: String,
    },
}
