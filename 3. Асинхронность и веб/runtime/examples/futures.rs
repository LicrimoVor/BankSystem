use futures::future::join_all;
use std::time::Duration;
use tokio::time::sleep;

// напишите здесь асинхронную функцию handle_connections
async fn handle_connections<F>(connections: Vec<F>)
where
    F: Future + Send + 'static,
    F::Output: std::marker::Send,
{
    let joins = connections
        .into_iter()
        .map(|c| tokio::spawn(c))
        .collect::<Vec<_>>();

    let _ = join_all(joins).await;
}

// Тесты
#[tokio::main]
async fn main() {
    use std::time::Instant;

    let connections = {
        let mut connections = Vec::new();
        for i in 0..10 {
            let connection = async move {
                sleep(Duration::from_millis(100)).await;
                println!("Hello from connection {i}");
            };
            connections.push(connection);
        }
        connections
    };

    let start = Instant::now();
    handle_connections(connections).await;
    let end = start.elapsed();

    assert!(end < Duration::from_millis(500))
}
