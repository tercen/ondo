// restore_reserved_tasks.rs
use super::TaskQueue;

impl TaskQueue {
    pub async fn restore_reserved_tasks(&self) {
        let mut tasks = self.tasks.write().await;
        let mut reserved = self.reserved.write().await;
        reserved.append(&mut tasks);
    }
}
