use super::{Extractor, SLEEP_TIME};
use crate::{
    logging,
    types::stock::{StockQuote, Ticker},
};
use std::{
    sync::mpsc::{Receiver, Sender},
    time,
};

/// Экстрактор случайных данных
pub struct RandomExtractor {
    subscribers: Vec<Sender<StockQuote>>,
    tickers: Vec<Ticker>,
}

impl RandomExtractor {
    pub fn new() -> Self {
        let tickers: Vec<Ticker> = (0..10).into_iter().map(|i| format!("T{}", i)).collect();

        Self {
            subscribers: Vec::new(),
            tickers,
        }
    }
}

impl Extractor for RandomExtractor {
    fn run(self: Box<Self>) -> Result<(), String> {
        logging!(info, ("RandomExtractor запущен"));

        loop {
            let Ok(timestamp) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
                logging!(warn, ("Невозможно получить время"));
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

            // logging!(info, ("Extractor: {:?}", quote));

            for tx in self.subscribers.iter() {
                if let Err(_e) = tx.send(quote.clone()) {
                    logging!(warn, ("Extractor: Ошибка при отправке: {}", _e));
                };
            }
            std::thread::sleep(SLEEP_TIME);
        }
    }

    fn subscribe(&mut self) -> Receiver<StockQuote> {
        let (tx, rx) = std::sync::mpsc::channel();
        self.subscribers.push(tx);
        rx
    }
}
