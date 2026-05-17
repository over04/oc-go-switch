use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyStatus {
    /// 当前最近一次被选中的 key。
    Active,
    /// 可展示但当前未激活的 key。
    Idle,
}

#[derive(Debug, Clone, Serialize)]
pub struct PoolKey {
    /// 调度内部 key id，包含账户和工作区前缀。
    pub id: String,
    /// 真实 API key，仅用于上游请求。
    pub key_value: String,
    /// OpenCode 页面中的 key 名称。
    pub key_name: String,
}

impl PoolKey {
    /// 脱敏显示 key：显示前 6 位和后 4 位。
    pub fn masked_key(&self) -> String {
        Self::mask_value(&self.key_value)
    }

    /// 静态工具方法：对任意 key 字符串脱敏。
    pub fn mask_value(raw: &str) -> String {
        if raw.len() <= 12 {
            return "***".to_string();
        }
        let prefix: String = raw.chars().take(6).collect();
        let suffix: String = raw
            .chars()
            .rev()
            .take(4)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect();
        format!("{prefix}...{suffix}")
    }
}
