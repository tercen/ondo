// spawn_workers.rs
use super::TaskQueue;
use std::sync::Arc;

impl TaskQueue {
    pub fn spawn_workers(queue: Arc<Self>, num_workers: usize) {
        for worker_id in 1..=num_workers {
            let queue_clone = Arc::clone(&queue);
            let queue_clone2 = Arc::clone(&queue);
            let handle = tokio::spawn(async move {
                worker_loop(queue_clone, worker_id).await;
            });

            tokio::spawn(async move {
                let mut worker_handles = queue_clone2.worker_handles.write().await;
                worker_handles.push(handle);
            });
        }
    }
}

async fn worker_loop(queue: Arc<TaskQueue>, worker_id: usize) {
    while !queue.should_stop().await {
        if let Some(task) = queue.dequeue().await {
            println!(
                "Worker {} processing task {}",
                worker_id,
                task.id.to_string()
            );
            if task.execute().await.is_ok() {
                queue.remove_reserved(&task.id).await;
            } else {
                queue.add_to_dead(task.clone()).await;
                println!(
                    "Worker {} moved task {} to dead tasks",
                    worker_id,
                    task.id.to_string()
                );
            }
        } else {
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    }
    println!("Worker {} stopped.", worker_id);
}
