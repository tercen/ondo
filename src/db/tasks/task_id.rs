// task_id.rs
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, PartialEq)]
pub struct TaskId {
    uuid: Uuid,
    timestamp: i64,
}

impl TaskId {
    pub fn new() -> Self {
        TaskId {
            uuid: Uuid::new_v4(),
            timestamp: Utc::now().timestamp_millis(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}-{}",
            self.uuid.simple().to_string()[..4].to_owned(),
            self.timestamp
        )
    }
}
