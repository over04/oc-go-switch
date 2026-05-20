use std::sync::Arc;

use arc_swap::ArcSwap;
use tokio::sync::Notify;

use crate::common::config::runtime::RuntimeConfig;

/// 热配置运行时容器。
///
/// 请求路径通过原子读取得到当前配置；配置更新时替换整份 `RuntimeConfig`。
#[derive(Debug, Clone)]
pub struct ConfigRuntime {
    current: Arc<ArcSwap<RuntimeConfig>>,
    changed: Arc<Notify>,
}

impl ConfigRuntime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            current: Arc::new(ArcSwap::from_pointee(config)),
            changed: Arc::new(Notify::new()),
        }
    }

    pub fn current(&self) -> Arc<RuntimeConfig> {
        self.current.load_full()
    }

    pub fn replace(&self, config: RuntimeConfig) {
        self.current.store(Arc::new(config));
        self.changed.notify_waiters();
    }

    pub async fn changed(&self) {
        self.changed.notified().await;
    }
}

#[cfg(test)]
mod tests {
    use crate::common::config::{runtime::RuntimeConfig, runtime_store::ConfigRuntime, Config};

    fn runtime_config(token: &str) -> RuntimeConfig {
        let yaml = format!(
            r#"
fixed:
  listen: 127.0.0.1:8180
runtime:
  accounts: []
  refresh_interval_secs: 300
  max_retries: 8
  go:
    base_url: https://opencode.ai/zen/go/v1
    connect_timeout_secs: 90
    request_timeout_secs: 90
  image_filter:
    models: []
  api_token: {token}
"#
        );
        Config::from_yaml(&yaml).unwrap().runtime
    }

    #[test]
    fn replace_publishes_new_runtime_config() {
        let runtime = ConfigRuntime::new(runtime_config("first-token"));
        assert_eq!(runtime.current().api_token, "first-token");

        runtime.replace(runtime_config("second-token"));

        assert_eq!(runtime.current().api_token, "second-token");
    }
}
