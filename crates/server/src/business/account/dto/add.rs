use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccountAddReqDto {
    pub name: String,
    pub auth: String,
    pub label: String,
}
