use regex::Regex;
use reqwest::Client;

use super::serverfn::{self, ServerFnError};
use super::types::{ApiKeyEntry, BillingInfo, GoUsage, SubscriptionPlan, Workspace};

const GET_WORKSPACES_HASH: &str =
    "def39973159c7f0483d8793a822b8dbb10d067e12c65455fcb4608459ba0234f";

pub struct OpencodeClient {
    client: Client,
    #[allow(dead_code)]
    pub account_name: String,
}

impl OpencodeClient {
    pub fn new(account_name: &str, auth_cookie: &str) -> Self {
        let cookie_raw = format!("auth={}", auth_cookie);
        let client = Client::builder()
            .cookie_store(true)
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::COOKIE,
                    reqwest::header::HeaderValue::from_str(&cookie_raw).unwrap(),
                );
                headers
            })
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client,
            account_name: account_name.to_string(),
        }
    }

    /// Discover all workspaces under this account.
    pub async fn get_workspaces(&self) -> Result<Vec<Workspace>, ServerFnError> {
        serverfn::call_no_args::<Vec<Workspace>>(&self.client, GET_WORKSPACES_HASH).await
    }

    /// Fetch the workspace page HTML.
    async fn fetch_workspace_page(&self, workspace_id: &str) -> Result<String, ServerFnError> {
        let url = format!("https://opencode.ai/workspace/{}", workspace_id);
        let resp = self.client.get(&url).send().await?;
        let text = resp.text().await?;
        Ok(text)
    }

    /// Fetch the Go usage page HTML for a workspace.
    async fn fetch_go_page(&self, workspace_id: &str) -> Result<String, ServerFnError> {
        let url = format!("https://opencode.ai/workspace/{}/go", workspace_id);
        let resp = self.client.get(&url).send().await?;
        let text = resp.text().await?;
        Ok(text)
    }

    /// Scrape API key from workspace page HTML.
    pub async fn list_keys(&self, workspace_id: &str) -> Result<Vec<ApiKeyEntry>, ServerFnError> {
        let html = self.fetch_workspace_page(workspace_id).await?;
        let re = Regex::new(r#"sk-[a-zA-Z0-9]{40,80}"#).unwrap();

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

    /// Scrape billing/subscription info from workspace page HTML.
    pub async fn get_billing(&self, workspace_id: &str) -> Result<BillingInfo, ServerFnError> {
        let html = self.fetch_workspace_page(workspace_id).await?;

        let plan = if html.contains(r#"plan:\"lite\""#) || html.contains("plan:\"lite\"") {
            Some(SubscriptionPlan::Go)
        } else if html.contains(r#"plan:\"zen\""#) || html.contains("plan:\"zen\"") {
            Some(SubscriptionPlan::Zen)
        } else {
            None
        };

        let balance = extract_balance(&html);

        let subscribed = plan == Some(SubscriptionPlan::Go);

        Ok(BillingInfo {
            balance,
            plan,
            subscribed,
        })
    }

    /// Scrape Go usage data from the workspace Go page.
    /// Only call for workspaces known to have a Go subscription.
    pub async fn get_go_usage(&self, workspace_id: &str) -> Result<Option<GoUsage>, ServerFnError> {
        let html = self.fetch_go_page(workspace_id).await?;

        // Parse rolling usage blocks: {status:"ok",resetInSec:NNN,usagePercent:NN}
        let re = Regex::new(
            r#"\bresetInSec:(\d+),usagePercent:(\d+)"#,
        )
        .unwrap();

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

/// Extract balance value from workspace page HTML.
/// Balance appears as `balance:<number>` in the $R stream data.
fn extract_balance(html: &str) -> i64 {
    let re = Regex::new(r#"balance:(-?\d+)"#).unwrap();
    if let Some(cap) = re.captures(html) {
        return cap[1].parse::<i64>().unwrap_or(0);
    }
    0
}
