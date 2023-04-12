// remove_reserved.rs
use super::TaskQueue;
use crate::db::tasks::task_id::TaskId;

impl TaskQueue {
    pub async fn remove_reserved(&self, task_id: &TaskId) {
        let mut reserved = self.reserved.write().await;
        reserved.retain(|task| task.id != *task_id);
    }
}
