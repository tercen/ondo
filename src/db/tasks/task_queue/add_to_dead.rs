// add_to_dead.rs
use super::TaskQueue;
use crate::db::tasks::task::Task;

impl TaskQueue {
    pub async fn add_to_dead(&self, task: Task) {
        let mut dead = self.dead.write().await;
        dead.push(task);
    }
}
