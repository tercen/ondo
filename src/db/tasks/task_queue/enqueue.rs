// enqueue.rs
use super::TaskQueue;
use crate::db::tasks::task::Task;

impl TaskQueue {
    pub async fn enqueue(&self, task: Task) {
        self.tasks.write().await.push(task);
    }
}
