// should_stop.rs
use super::TaskQueue;

impl TaskQueue {
    pub async fn should_stop(&self) -> bool {
        *self.stop_flag.read().await
    }
}
