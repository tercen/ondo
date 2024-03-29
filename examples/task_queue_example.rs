// main.rs
use ondo::db::tasks::task::{Task};
use ondo::db::tasks::task_queue::TaskQueue;

#[tokio::main]
async fn main() {
    let queue = TaskQueue::new();

    // Restore reserved tasks
    queue.restore_reserved_tasks().await;

    // Spawn 3 worker tasks
    TaskQueue::spawn_workers(queue.clone(), 3);

    // Add tasks to the queue
    for i in 1..=10 {
        let task = if i % 2 == 0 {
            Task::new_type_a(format!("Task {}", i), "worker1".to_string()) 
        } else {
            Task::new_type_b(format!("Task {}", i), "worker2".to_string())
        };
        queue.enqueue(task).await;
    }

    // Wait for a while before stopping the workers
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    // Stop and wait for the TaskQueue to finish before the end of the program
    queue.stop_and_wait().await;
}
