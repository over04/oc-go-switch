use regex::Regex;
use reqwest::Client;
use std::sync::OnceLock;

use super::serverfn::{self, ServerFnError};
use super::types::{ApiKeyEntry, BillingInfo, GoUsage, SubscriptionPlan, Workspace};

const GET_WORKSPACES_HASH: &str =
    "def39973159c7f0483d8793a822b8dbb10d067e12c65455fcb4608459ba0234f";

static KEY_REGEX: OnceLock<Regex> = OnceLock::new();
static USAGE_REGEX: OnceLock<Regex> = OnceLock::new();

pub struct OpencodeClient {
    client: Client,
    #[allow(dead_code)]
    pub account_name: String,
}

impl OpencodeClient {
    pub fn new(account_name: &str, auth_cookie: &str) -> Result<Self, String> {
        let cookie_raw = format!("auth={}", auth_cookie);
        let header_value = reqwest::header::HeaderValue::from_str(&cookie_raw)
            .map_err(|e| format!("无效的 auth cookie: {e}"))?;
        let client = Client::builder()
            .cookie_store(true)
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(reqwest::header::COOKIE, header_value);
                headers
            })
            .build()
            .map_err(|e| format!("构建 reqwest client 失败: {e}"))?;

        Ok(Self {
            client,
            account_name: account_name.to_string(),
        })
    }

    /// 发现该账户下的所有工作区。
    pub async fn get_workspaces(&self) -> Result<Vec<Workspace>, ServerFnError> {
        serverfn::call_no_args::<Vec<Workspace>>(&self.client, GET_WORKSPACES_HASH).await
    }

    /// 获取工作区页面 HTML。
    async fn fetch_workspace_page(&self, workspace_id: &str) -> Result<String, ServerFnError> {
        let url = format!("https://opencode.ai/workspace/{}", workspace_id);
        let resp = self.client.get(&url).send().await?;
        let text = resp.text().await?;
        Ok(text)
    }

    /// 获取工作区 Go 用量页面 HTML。
    async fn fetch_go_page(&self, workspace_id: &str) -> Result<String, ServerFnError> {
        let url = format!("https://opencode.ai/workspace/{}/go", workspace_id);
        let resp = self.client.get(&url).send().await?;
        let text = resp.text().await?;
        Ok(text)
    }

    /// 从工作区页面 HTML 刮取 API key。
    pub async fn list_keys(&self, workspace_id: &str) -> Result<Vec<ApiKeyEntry>, ServerFnError> {
        let html = self.fetch_workspace_page(workspace_id).await?;
        let re = KEY_REGEX
            .get_or_init(|| Regex::new(r#"sk-[a-zA-Z0-9]{40,80}"#).expect("hardcoded key regex"));
        let keys: Vec<ApiKeyEntry> = re
            .find_iter(&html)
            .map(|m| {
                let key = m.as_str().to_string();
                ApiKeyEntry {
                    id: key.clone(),
                    key,
                    name: "default".to_string(),
                    created_at: String::new(),
                }
            })
            .collect();

        Ok(keys)
    }

    /// 从工作区页面 HTML 刮取订阅/账单信息。
    pub async fn get_billing(&self, workspace_id: &str) -> Result<BillingInfo, ServerFnError> {
        let html = self.fetch_workspace_page(workspace_id).await?;

        // Go 订阅：liteSubscriptionID 非空（如 liteSubscriptionID:"sub_xxx"）
        let has_go = html.contains("liteSubscriptionID:\"sub_");

        let plan = if has_go {
            Some(SubscriptionPlan::Go)
        } else if html.contains("subscriptionPlan:\"zen\"") {
            Some(SubscriptionPlan::Zen)
        } else {
            None
        };

        let subscribed = has_go;

        Ok(BillingInfo { plan, subscribed })
    }

    /// 从工作区 Go 页面刮取 Go 用量数据。
    pub async fn get_go_usage(&self, workspace_id: &str) -> Result<Option<GoUsage>, ServerFnError> {
        let html = self.fetch_go_page(workspace_id).await?;

        let re = USAGE_REGEX.get_or_init(|| {
            Regex::new(r#"\bresetInSec:(\d+),usagePercent:(\d+)"#)
                .expect("hardcoded go_usage regex")
        });
        let mut matches: Vec<(u64, u32)> = Vec::new();
        for caps in re.captures_iter(&html) {
            if matches.len() >= 3 {
                break;
            }
            let reset: u64 = caps[1].parse().unwrap_or(0);
            let percent: u32 = caps[2].parse().unwrap_or(0);
            matches.push((reset, percent));
        }

        if matches.len() < 3 {
            return Ok(None);
        }

        Ok(Some(GoUsage {
            hourly_percent: matches[0].1,
            hourly_reset_sec: matches[0].0,
            weekly_percent: matches[1].1,
            weekly_reset_sec: matches[1].0,
            monthly_percent: matches[2].1,
            monthly_reset_sec: matches[2].0,
        }))
    }
}
