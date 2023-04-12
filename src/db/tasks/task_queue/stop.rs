// stop.rs
use super::TaskQueue;

impl TaskQueue {
    pub async fn stop(&self) {
        *self.stop_flag.write().await = true;
    }
}
