use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccountListEntryDto {
    /// 账户唯一名称。
    pub name: String,
    /// 前端展示标签。
    pub label: String,
    /// 脱敏后的 auth cookie。
    pub auth_masked: String,
}

#[derive(Debug, Serialize)]
pub struct AccountListRespDto {
    /// 已配置账户列表。
    pub accounts: Vec<AccountListEntryDto>,
}
