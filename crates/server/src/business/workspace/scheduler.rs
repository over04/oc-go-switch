use std::collections::{HashMap, VecDeque};

use crate::business::workspace::{
    key::PoolKey, record::WorkspacePool, status::WorkspacePoolStatus,
};

/// 工作区调度状态。
///
/// 完整工作区数据保存在 `workspaces`，调度队列只保存 available 工作区 id。
/// Go 额度按工作区共享，因此出队和轮询都以工作区为单位。
#[derive(Debug)]
pub struct KeyPool {
    pub workspaces: HashMap<String, WorkspacePool>,
    pub workspace_queue: VecDeque<String>,
    pub current_workspace_id: Option<String>,
    pub current_key_id: Option<String>,
    pub last_refresh_at: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SelectedKey {
    pub id: String,
    pub key_value: String,
    pub workspace_id: String,
    pub workspace_name: String,
}

impl SelectedKey {
    pub fn masked_key(&self) -> String {
        PoolKey::mask_value(&self.key_value)
    }
}

impl KeyPool {
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
            current_key_id: None,
            last_refresh_at: None,
        }
    }

    pub fn available_workspace_count(&self) -> usize {
        self.workspace_queue.len()
    }

    pub fn has_available_workspace(&self) -> bool {
        self.workspaces
            .values()
            .any(|workspace| workspace.status == WorkspacePoolStatus::Available)
    }

    pub fn select(&mut self) -> Option<SelectedKey> {
        let workspace_id = self.workspace_queue.pop_front()?;
        let key = {
            let workspace = self.workspaces.get_mut(&workspace_id)?;
            workspace.keys.pop_front()?
        };

        let selected = {
            let workspace = self.workspaces.get(&workspace_id)?;
            SelectedKey {
                id: key.id.clone(),
                key_value: key.key_value.clone(),
                workspace_id: workspace.id.clone(),
                workspace_name: workspace.name.clone(),
            }
        };

        let workspace = self.workspaces.get_mut(&workspace_id)?;
        workspace.keys.push_back(key);
        self.workspace_queue.push_back(workspace_id.clone());
        self.current_workspace_id = Some(workspace_id);
        self.current_key_id = Some(selected.id.clone());
        Some(selected)
    }

    pub fn set_active_key(&mut self, key_id: &str) -> bool {
        let Some((workspace_id, key)) = self.take_key(key_id) else {
            return false;
        };

        self.current_workspace_id = Some(workspace_id.clone());
        self.current_key_id = Some(key.id.clone());
        if let Some(workspace) = self.workspaces.get_mut(&workspace_id) {
            workspace.keys.push_front(key);
        }
        self.move_workspace_front(&workspace_id);
        true
    }

    pub fn clear_active_key(&mut self) {
        self.current_workspace_id = None;
        self.current_key_id = None;
    }

    pub fn drop_workspace(&mut self, workspace_id: &str) -> bool {
        self.workspaces.remove(workspace_id);
        self.workspace_queue.retain(|id| id != workspace_id);
        if self.current_workspace_id.as_deref() == Some(workspace_id) {
            self.clear_active_key();
        }
        self.workspace_queue.is_empty()
    }

    fn take_key(&mut self, key_id: &str) -> Option<(String, PoolKey)> {
        for (workspace_id, workspace) in &mut self.workspaces {
            if let Some(index) = workspace.keys.iter().position(|key| key.id == key_id) {
                let key = workspace.keys.remove(index)?;
                return Some((workspace_id.clone(), key));
            }
        }
        None
    }

    fn move_workspace_front(&mut self, workspace_id: &str) {
        self.workspace_queue.retain(|id| id != workspace_id);
        self.workspace_queue.push_front(workspace_id.to_string());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};

    use crate::business::workspace::{
        key::PoolKey, record::WorkspacePool, scheduler::KeyPool, status::WorkspacePoolStatus,
    };
    use adapter::opencode::model::{go_usage::GoUsage, subscription_plan::SubscriptionPlan};

    #[test]
    fn queue_contains_available_workspaces_only() {
        let pool = KeyPool::new(HashMap::from([
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
    fn select_rotates_workspace_and_key() {
        let mut pool = KeyPool::new(HashMap::from([workspace(
            "available",
            WorkspacePoolStatus::Available,
            20,
        )]));

        let first = pool.select().expect("available workspace should select");
        let second = pool.select().expect("available workspace should rotate");

        assert_eq!(first.workspace_id, "available");
        assert_eq!(second.workspace_id, "available");
        assert_ne!(first.id, second.id);
    }

    #[test]
    fn drop_workspace_removes_it_from_scheduler() {
        let mut pool = KeyPool::new(HashMap::from([workspace(
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
                keys: VecDeque::from([
                    PoolKey {
                        id: format!("{id}/key-1"),
                        key_value: "sk-11111111111111111111".to_string(),
                        key_name: "key-1".to_string(),
                    },
                    PoolKey {
                        id: format!("{id}/key-2"),
                        key_value: "sk-22222222222222222222".to_string(),
                        key_name: "key-2".to_string(),
                    },
                ]),
            },
        )
    }
}
