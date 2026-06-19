use std::collections::{HashMap, VecDeque};

use crate::business::workspace::{
    credential::WorkspaceCredential, record::WorkspacePool, status::WorkspacePoolStatus,
};

/// 工作区调度状态。
///
/// 完整工作区数据保存在 `workspaces`，调度队列只保存 available 工作区 id。
/// Go 额度按工作区共享；同一工作区内多个 OpenCode key 等价，只保存一个代理凭证。
#[derive(Debug)]
pub struct WorkspaceScheduler {
    pub workspaces: HashMap<String, WorkspacePool>,
    pub workspace_queue: VecDeque<String>,
    pub current_workspace_id: Option<String>,
    pub last_refresh_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SelectedWorkspaceCredential {
    pub credential_value: String,
    pub workspace_id: String,
    pub workspace_name: String,
}

impl SelectedWorkspaceCredential {
    pub fn masked_credential(&self) -> String {
        WorkspaceCredential::mask_value(&self.credential_value)
    }
}

impl WorkspaceScheduler {
    pub fn new(workspaces: HashMap<String, WorkspacePool>) -> Self {
        let mut workspace_queue: VecDeque<String> = workspaces
            .iter()
            .filter(|(_, workspace)| workspace.status == WorkspacePoolStatus::Available)
            .map(|(id, _)| id.clone())
            .collect();
        workspace_queue.make_contiguous().sort_by_key(|id| {
            workspaces
                .get(id)
                .map_or(u32::MAX, WorkspacePool::usage_rank)
        });

        Self {
            workspaces,
            workspace_queue,
            current_workspace_id: None,
            last_refresh_at: None,
        }
    }

    pub fn available_workspace_count(&self) -> usize {
        self.workspace_queue.len()
    }

    pub fn select(&mut self) -> Option<SelectedWorkspaceCredential> {
        if let Some(workspace_id) = self.current_workspace_id.clone() {
            if self.is_available_workspace(&workspace_id) {
                if let Some(selected) = self.select_from_workspace(&workspace_id) {
                    return Some(selected);
                }
            }
            self.current_workspace_id = None;
        }

        let workspace_id = self.workspace_queue.pop_front()?;
        let selected = self.select_from_workspace(&workspace_id)?;
        self.workspace_queue.push_back(workspace_id);
        Some(selected)
    }

    pub fn restore_current_workspace(&mut self, workspace_id: &str) -> bool {
        if !self.is_available_workspace(workspace_id) {
            self.current_workspace_id = None;
            return false;
        }

        self.current_workspace_id = Some(workspace_id.to_string());
        self.move_workspace_front(workspace_id);
        true
    }

    pub fn set_current_workspace(&mut self, workspace_id: &str) -> bool {
        self.restore_current_workspace(workspace_id)
    }

    pub fn clear_current_workspace(&mut self) {
        self.current_workspace_id = None;
    }

    pub fn drop_workspace(&mut self, workspace_id: &str) -> bool {
        self.workspaces.remove(workspace_id);
        self.workspace_queue.retain(|id| id != workspace_id);
        if self.current_workspace_id.as_deref() == Some(workspace_id) {
            self.current_workspace_id = None;
        }
        self.workspace_queue.is_empty()
    }

    fn select_from_workspace(&mut self, workspace_id: &str) -> Option<SelectedWorkspaceCredential> {
        let workspace = self.workspaces.get(workspace_id)?;
        if workspace.status != WorkspacePoolStatus::Available {
            return None;
        }

        let selected = SelectedWorkspaceCredential {
            credential_value: workspace.credential.value.clone(),
            workspace_id: workspace.id.clone(),
            workspace_name: workspace.name.clone(),
        };
        self.current_workspace_id = Some(workspace_id.to_string());
        Some(selected)
    }

    fn move_workspace_front(&mut self, workspace_id: &str) {
        self.workspace_queue.retain(|id| id != workspace_id);
        self.workspace_queue.push_front(workspace_id.to_string());
    }

    fn is_available_workspace(&self, workspace_id: &str) -> bool {
        self.workspaces
            .get(workspace_id)
            .is_some_and(|workspace| workspace.status == WorkspacePoolStatus::Available)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};

    use crate::business::workspace::{
        credential::WorkspaceCredential, record::WorkspacePool, scheduler::WorkspaceScheduler,
        status::WorkspacePoolStatus,
    };
    use adapter::opencode::model::{go_usage::GoUsage, subscription_plan::SubscriptionPlan};

    #[test]
    fn queue_contains_available_workspaces_only() {
        let pool = WorkspaceScheduler::new(HashMap::from([
            workspace("available", WorkspacePoolStatus::Available, 20),
            workspace("exhausted", WorkspacePoolStatus::Exhausted, 100),
            workspace("unsubscribed", WorkspacePoolStatus::Unsubscribed, 0),
        ]));

        assert_eq!(
            pool.workspace_queue,
            VecDeque::from(["available".to_string()])
        );
    }

    #[test]
    fn select_uses_queue_when_current_is_empty() {
        let mut pool = WorkspaceScheduler::new(HashMap::from([
            workspace("workspace-a", WorkspacePoolStatus::Available, 20),
            workspace("workspace-b", WorkspacePoolStatus::Available, 10),
        ]));

        let first = pool.select().expect("available workspace should select");
        pool.clear_current_workspace();
        let second = pool.select().expect("available workspace should select");

        assert_eq!(first.workspace_id, "workspace-b");
        assert_eq!(second.workspace_id, "workspace-a");
    }

    #[test]
    fn select_sticks_to_current_workspace() {
        let mut pool = WorkspaceScheduler::new(HashMap::from([
            workspace("workspace-a", WorkspacePoolStatus::Available, 20),
            workspace("workspace-b", WorkspacePoolStatus::Available, 10),
        ]));

        let selected_workspaces: Vec<String> = (0..4)
            .map(|_| {
                pool.select()
                    .expect("current workspace should select")
                    .workspace_id
            })
            .collect();

        assert_eq!(
            selected_workspaces,
            vec!["workspace-b", "workspace-b", "workspace-b", "workspace-b"]
        );
        assert_eq!(pool.current_workspace_id.as_deref(), Some("workspace-b"));
    }

    #[test]
    fn restore_current_keeps_available_workspace_at_front() {
        let mut pool = WorkspaceScheduler::new(HashMap::from([
            workspace("workspace-a", WorkspacePoolStatus::Available, 50),
            workspace("workspace-b", WorkspacePoolStatus::Available, 10),
        ]));

        assert!(pool.restore_current_workspace("workspace-a"));

        assert_eq!(pool.current_workspace_id.as_deref(), Some("workspace-a"));
        assert_eq!(
            pool.workspace_queue.front().map(String::as_str),
            Some("workspace-a")
        );
    }

    #[test]
    fn restore_current_clears_unavailable_workspace() {
        let mut pool = WorkspaceScheduler::new(HashMap::from([
            workspace("workspace-a", WorkspacePoolStatus::Exhausted, 100),
            workspace("workspace-b", WorkspacePoolStatus::Available, 10),
        ]));
        pool.current_workspace_id = Some("workspace-a".to_string());

        assert!(!pool.restore_current_workspace("workspace-a"));

        assert_eq!(pool.current_workspace_id, None);
        assert_eq!(
            pool.workspace_queue.front().map(String::as_str),
            Some("workspace-b")
        );
    }

    #[test]
    fn drop_workspace_clears_current_workspace() {
        let mut pool = WorkspaceScheduler::new(HashMap::from([
            workspace("workspace-a", WorkspacePoolStatus::Available, 20),
            workspace("workspace-b", WorkspacePoolStatus::Available, 10),
        ]));
        pool.current_workspace_id = Some("workspace-a".to_string());

        assert!(!pool.drop_workspace("workspace-a"));

        assert_eq!(pool.current_workspace_id, None);
        assert_eq!(
            pool.workspace_queue,
            VecDeque::from(["workspace-b".to_string()])
        );
    }

    #[test]
    fn drop_workspace_removes_it_from_scheduler() {
        let mut pool = WorkspaceScheduler::new(HashMap::from([workspace(
            "available",
            WorkspacePoolStatus::Available,
            20,
        )]));

        assert!(pool.drop_workspace("available"));
        assert!(pool.workspace_queue.is_empty());
        assert!(pool.workspaces.is_empty());
    }

    fn workspace(
        id: &'static str,
        status: WorkspacePoolStatus,
        usage: u32,
    ) -> (String, WorkspacePool) {
        let id = id.to_string();
        (
            id.clone(),
            WorkspacePool {
                id: id.clone(),
                name: id.clone(),
                account_name: "acct".to_string(),
                account_label: "Account".to_string(),
                status,
                plan: Some(SubscriptionPlan::Go),
                go_usage: Some(GoUsage {
                    hourly_percent: usage,
                    hourly_reset_sec: 1,
                    weekly_percent: usage,
                    weekly_reset_sec: 1,
                    monthly_percent: usage,
                    monthly_reset_sec: 1,
                }),
                credential: WorkspaceCredential {
                    value: format!("sk-{id}-credential"),
                },
            },
        )
    }
}
