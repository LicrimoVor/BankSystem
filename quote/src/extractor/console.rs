use log::{info, warn};

use super::{Extractor, SLEEP_TIME};
use crate::types::stock::StockQuote;
use std::{
    io::{self, Write},
    sync::mpsc::{Receiver, Sender},
    time,
};

pub struct ConsoleExtractor {
    subscribers: Vec<Sender<StockQuote>>,
}

impl Extractor for ConsoleExtractor {
    fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }
    fn run(self) -> Result<Self, String> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut buf = String::new();
        stdout
            .write_all("Формат: <ticker> <price> <volume>\nНапример: BCD 100 0".as_bytes())
            .unwrap();

        #[cfg(feature = "logging")]
        info!("ConsoleExtractor запущен");

        loop {
            stdin.read_line(&mut buf).unwrap();
            if buf.trim().to_uppercase() == "EXIT" {
                break;
            }

            let Ok(timestamp) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
                #[cfg(feature = "logging")]
                warn!("Невозможно получить время");
                continue;
            };

            let parts: Vec<&str> = buf.trim().split_whitespace().collect();
            if parts.len() != 3 {
                #[cfg(feature = "logging")]
                warn!("Неверное количество аргументов");
                stdout
                    .write_all("Неверный формат".as_bytes())
                    .map_err(|e| e.to_string())?;
                continue;
            }

            let Ok(price): Result<f64, _> = parts[1].parse() else {
                #[cfg(feature = "logging")]
                warn!("Неверный формат ввода price");
                stdout
                    .write_all("еверный формат ввода цены".as_bytes())
                    .map_err(|e| e.to_string())?;
                continue;
            };

            let Ok(volume): Result<u32, _> = parts[2].parse() else {
                #[cfg(feature = "logging")]
                warn!("Неверный формат ввода volume");
                stdout
                    .write_all("Неверный формат ввода объема".as_bytes())
                    .map_err(|e| e.to_string())?;
                continue;
            };

            let quote = StockQuote {
                timestamp: timestamp.as_secs(),
                ticker: parts[0].to_string(),
                price: price,
                volume: volume,
            };

            #[cfg(feature = "logging")]
            info!("Получена котировка: {:?}", quote);

            for tx in self.subscribers.iter() {
                if let Err(e) = tx.send(quote.clone()) {
                    #[cfg(feature = "logging")]
                    warn!("Ошибка при отправке: {}", e);
                    stdout
                        .write_all("Ошибка при отправке".as_bytes())
                        .map_err(|e| e.to_string())?;
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
