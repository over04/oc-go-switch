use serde::Serialize;

use crate::{
    business::account::dto::list::AccountListEntryDto,
    common::config::{go::GoConfig, image_filter::ImageFilterConfig},
};

#[derive(Debug, Serialize)]
pub struct ConfigurationGetRespDto {
    /// 服务监听地址。
    pub listen: String,
    /// 后台刷新间隔，单位秒。
    pub refresh_interval_secs: u64,
    /// 单请求最大重试次数。
    pub max_retries: usize,
    /// Go 上游配置。
    pub go: GoConfig,
    /// 脱敏后的账户列表。
    pub accounts: Vec<AccountListEntryDto>,
    /// 图片过滤配置。
    pub image_filter: ImageFilterConfig,
    /// 只暴露是否已设置 token，不返回原文。
    pub api_token_set: bool,
}
