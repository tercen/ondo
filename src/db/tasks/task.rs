// task.rs
use crate::db::entity::text_index::text_index_task::TextIndexTask;
use crate::db::tasks::task_id::TaskId;
use tokio::time::Duration;

#[derive(Debug, Clone)]
pub struct Task {
    pub id: TaskId,
    pub worker_name: String,
    task_type: TaskType,
}

pub(crate) type Tasks = Vec<Task>;

#[derive(Debug, Clone)]
pub(crate) enum TaskType {
    TypeA { data: String },
    TypeB { data: String },
    TextIndex(TextIndexTask),
}

impl Task {
    pub fn new_type_a(data: String, worker_name: String) -> Self {
        let id = TaskId::new();
        Task {
            id,
            worker_name,
            task_type: TaskType::TypeA { data },
        }
    }

    pub fn new_type_b(data: String, worker_name: String) -> Self {
        let id = TaskId::new();
        Task {
            id,
            worker_name,
            task_type: TaskType::TypeB { data },
        }
    }

    pub(crate) fn from_text_index_task(text_index_task: TextIndexTask) -> Self {
        let id = TaskId::new();
        Task {
            id,
            worker_name: "".to_string(),
            task_type: TaskType::TextIndex(text_index_task),
        }
    }

    pub async fn execute(&self) -> Result<(), String> {
        match &self.task_type {
            TaskType::TypeA { data } => {
                println!("Processing TaskTypeA {}: {}", self.id.to_string(), data);
                tokio::time::sleep(Duration::from_millis(100)).await;
                Ok(())
            }
            TaskType::TypeB { data } => {
                println!("Processing TaskTypeB {}: {}", self.id.to_string(), data);
                tokio::time::sleep(Duration::from_millis(200)).await;
                Ok(())
            }
            TaskType::TextIndex(text_index_task) => text_index_task.execute().await,
        }
    }
}
