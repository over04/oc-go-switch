#[derive(Debug, Clone)]
pub struct WorkspaceCredential {
    /// 真实 API key，仅用于上游请求。
    pub value: String,
}

impl WorkspaceCredential {
    /// 脱敏显示凭证：显示前 6 位和后 4 位。
    pub fn masked(&self) -> String {
        Self::mask_value(&self.value)
    }

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
