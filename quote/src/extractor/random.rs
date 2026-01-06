use super::{Extractor, SLEEP_TIME};
use crate::types::stock::{StockQuote, Ticker};
use log::{info, warn};
use std::{
    sync::mpsc::{Receiver, Sender},
    time,
};

/// Экстрактор случайных данных
pub struct RandomExtractor {
    subscribers: Vec<Sender<StockQuote>>,
    tickers: Vec<Ticker>,
}

impl Extractor for RandomExtractor {
    fn new() -> Self {
        let tickers: Vec<Ticker> = (0..10).into_iter().map(|i| format!("T{}", i)).collect();
        Self {
            subscribers: Vec::new(),
            tickers,
        }
    }
    fn run(self) -> Result<Self, String> {
        #[cfg(feature = "logging")]
        info!("RandomExtractor запущен");

        loop {
            let Ok(timestamp) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
                #[cfg(feature = "logging")]
                warn!("Невозможно получить время");
                continue;
            };
            let indx = rand::random_range(0..10);
            let ticker = self.tickers[indx].clone();

            let quote = StockQuote {
                timestamp: timestamp.as_secs(),
                ticker,
                price: rand::random(),
                volume: rand::random(),
            };

            // #[cfg(feature = "logging")]
            // info!("Extractor: {:?}", quote);

            for tx in self.subscribers.iter() {
                if let Err(e) = tx.send(quote.clone()) {
                    #[cfg(feature = "logging")]
                    warn!("Extractor: Ошибка при отправке: {}", e);
                };
            }
            std::thread::sleep(SLEEP_TIME);
        }
        Ok(self)
    }

    fn subscribe(&mut self) -> Receiver<StockQuote> {
        let (tx, rx) = std::sync::mpsc::channel();
        self.subscribers.push(tx);
        rx
    }
}
