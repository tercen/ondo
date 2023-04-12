// task.rs
use crate::db::tasks::task_id::TaskId;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub enum TaskType {
    TypeA,
    TypeB,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub(crate) id: TaskId,
    task_type: TaskType,
    data: String,
}

impl Task {
    pub fn new(task_type: TaskType, data: String) -> Self {
        let id = TaskId::new();
        Task { id, task_type, data }
    }

    pub async fn execute(&self) -> Result<(), String> {
        match &self.task_type {
            TaskType::TypeA => {
                println!("Processing TaskTypeA {}: {}", self.id.to_string(), self.data);
                tokio::time::sleep(Duration::from_millis(100)).await;
                Ok(())
            }
            TaskType::TypeB => {
                println!("Processing TaskTypeB {}: {}", self.id.to_string(), self.data);
                tokio::time::sleep(Duration::from_millis(200)).await;
                Ok(())
            }
        }
    }
}
