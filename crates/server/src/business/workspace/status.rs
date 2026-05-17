#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspacePoolStatus {
    /// 有 Go 订阅且当前仍有额度，可进入调度队列。
    Available,
    /// 有 Go 订阅但当前小时/周/月额度已有任一维度耗尽。
    Exhausted,
    /// 工作区缺少 Go 订阅，只展示状态，不参与调度。
    Unsubscribed,
}
