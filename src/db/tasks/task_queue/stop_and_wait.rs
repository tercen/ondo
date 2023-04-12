use super::TaskQueue;

impl TaskQueue {
    pub async fn stop_and_wait(&self) {
        self.stop().await;

        let join_futures = self
            .worker_handles
            .write()
            .await
            .drain(..)
            .collect::<Vec<_>>();
        futures::future::join_all(join_futures).await;
    }
}
