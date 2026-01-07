use super::{Extractor, SLEEP_TIME};
use crate::types::stock::{StockQuote, Ticker};
#[cfg(feature = "logging")]
use log::{info, warn};
use std::{
    fs::File,
    io::Read,
    sync::mpsc::{Receiver, Sender},
    time,
};

/// Экстрактор данных с файла
pub struct FileMockExtractor {
    subscribers: Vec<Sender<StockQuote>>,
    tickers: Vec<Ticker>,
}

impl FileMockExtractor {
    pub fn new(mut file: File) -> Self {
        let mut text = String::new();
        if let Err(e) = file.read_to_string(&mut text) {
            #[cfg(feature = "logging")]
            warn!("Невозможно прочитать файл: {}", e);
        };

        let tickers: Vec<Ticker> = text
            .lines()
            .into_iter()
            .map(|s| s.trim().to_string())
            .collect();

        Self {
            subscribers: Vec::new(),
            tickers,
        }
    }
}

impl Extractor for FileMockExtractor {
    fn run(self: Box<Self>) -> Result<(), String> {
        #[cfg(feature = "logging")]
        info!("FileMockExtractor запущен");
        let count = self.tickers.len();
        let mut indx: usize = 0;

        loop {
            let Ok(timestamp) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
                #[cfg(feature = "logging")]
                warn!("Невозможно получить время");
                continue;
            };

            let ticker = self.tickers[indx].clone();
            indx = (indx + 1) % count;

            let quote = StockQuote {
                timestamp: timestamp.as_secs(),
                ticker,
                price: (indx as f64) / 10.0,
                volume: indx as u32,
            };

            for tx in self.subscribers.iter() {
                if let Err(e) = tx.send(quote.clone()) {
                    #[cfg(feature = "logging")]
                    warn!("Extractor: Ошибка при отправке: {}", e);
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
