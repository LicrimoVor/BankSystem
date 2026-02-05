use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

async fn call_with_retry(
    client: &Client,
    url: &str,
    max_retries: u32,
) -> Result<String, reqwest::Error> {
    let mut last_error = None;

    for attempt in 0..=max_retries {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(response.text().await?);
                }
                // Если статус не успешный, пробуем ещё раз
            }
            Err(e) => {
                last_error = Some(e);
            }
        }

        if attempt < max_retries {
            // Экспоненциальная задержка: 1s, 2s, 4s
            let delay = Duration::from_secs(2_u64.pow(attempt));
            sleep(delay).await;
        }
    }

    Err(last_error.unwrap())
}
