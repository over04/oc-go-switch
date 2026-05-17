use thiserror::Error;

/// 配置加载与校验错误。
///
/// 这些错误发生在进程启动或保存配置时，属于必须向调用方明确暴露的边界错误。
#[derive(Debug, Error)]
pub enum ConfigError {
    /// YAML 语法或字段类型解析失败。
    #[error("解析配置文件失败: {0}")]
    Parse(#[from] serde_yaml::Error),
    /// 重试次数上限异常，防止单个请求在故障时长时间占用资源。
    #[error("max_retries 不能超过 100，当前值为 {0}")]
    MaxRetriesTooLarge(usize),
    /// 必填 token 为空或只包含空白字符。
    #[error("config.yaml 缺少有效的 {0}")]
    MissingToken(&'static str),
}
