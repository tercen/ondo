// task_queue.rs
mod add_to_dead;
mod dequeue;
mod enqueue;
mod remove_reserved;
mod restore_reserved_tasks;
mod should_stop;
mod spawn_workers;
mod stop;
mod stop_and_wait;

use crate::db::tasks::task::Task;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct TaskQueue {
    tasks: RwLock<Vec<Task>>,
    reserved: RwLock<Vec<Task>>,
    dead: RwLock<Vec<Task>>,
    stop_flag: RwLock<bool>,
    worker_handles: RwLock<Vec<tokio::task::JoinHandle<()>>>,
}

impl TaskQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(TaskQueue {
            tasks: RwLock::new(Vec::new()),
            reserved: RwLock::new(Vec::new()),
            dead: RwLock::new(Vec::new()),
            stop_flag: RwLock::new(false),
            worker_handles: RwLock::new(Vec::new()),
        })
    }
}
