use std::{task::Context, time::Duration};

struct WaitFor {
    duration: Duration,
    waited: bool,
}

fn wait_for(duration: Duration) -> WaitFor {
    WaitFor {
        duration,
        waited: false,
    }
}

impl Future for WaitFor {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.waited {
            return std::task::Poll::Ready(());
        }
        self.waited = true;
        let waker = cx.waker().clone();
        let duration = self.duration;
        std::thread::spawn(move || {
            std::thread::sleep(duration);
            waker.wake();
        });
        std::task::Poll::Pending
    }
}

#[tokio::main]
async fn main() {
    println!("Before wait");
    wait_for(std::time::Duration::from_secs(2)).await;
    println!("After wait");
}
