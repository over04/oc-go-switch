use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccountEditReqDto {
    pub auth: Option<String>,
    pub label: Option<String>,
}
