// dequeue.rs
use super::TaskQueue;
use crate::db::tasks::task::Task;

impl TaskQueue {
    pub async fn dequeue(&self) -> Option<Task> {
        let mut tasks = self.tasks.write().await;
        let mut reserved = self.reserved.write().await;
        if let Some(task) = tasks.pop() {
            reserved.push(task.clone());
            Some(task)
        } else {
            None
        }
    }
}
