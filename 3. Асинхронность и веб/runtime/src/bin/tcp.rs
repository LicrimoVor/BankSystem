use std::time::{Duration, Instant};

use tokio::{
    sync::mpsc::{Receiver, Sender, channel, error::SendError},
    time::timeout,
};

struct TaskPool<T> {
    tasks: Receiver<T>,
    sender: Sender<T>,
}

impl<T> TaskPool<T> {
    fn new(queue_size: usize) -> Self {
        let (sender, tasks) = channel::<T>(queue_size);
        Self { tasks, sender }
    }

    async fn create(&self, task: T) -> Option<Result<(), SendError<T>>> {
        timeout(Duration::from_millis(100), self.sender.send(task))
            .await
            .ok()
    }

    async fn pull_task(&mut self) -> Option<T> {
        self.tasks.recv().await
    }
}

#[tokio::main]
async fn main() {
    let mut tasks = TaskPool::new(2);

    assert_eq!(tasks.create(()).await, Some(Ok(())));
    assert_eq!(tasks.create(()).await, Some(Ok(())));

    let start = Instant::now();
    assert_eq!(tasks.create(()).await, None);
    let end = start.elapsed();

    assert!(end > Duration::from_millis(90));
    assert!(end < Duration::from_millis(110));

    assert_eq!(tasks.pull_task().await, Some(()));
    assert_eq!(tasks.create(()).await, Some(Ok(())));
}
