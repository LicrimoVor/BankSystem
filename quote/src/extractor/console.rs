use super::{Extractor, SLEEP_TIME};
use crate::{logging, types::stock::StockQuote};
use std::{
    io::{self, Write},
    sync::mpsc::{Receiver, Sender},
    time,
};

pub struct ConsoleExtractor {
    subscribers: Vec<Sender<StockQuote>>,
}

impl ConsoleExtractor {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }
}

impl Extractor for ConsoleExtractor {
    fn run(self: Box<Self>) -> Result<(), String> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let mut buf = String::new();
        stdout
            .write_all("Формат: <ticker> <price> <volume>\nНапример: BCD 100 0".as_bytes())
            .unwrap();

        logging!(info, ("ConsoleExtractor запущен"));

        loop {
            stdin.read_line(&mut buf).unwrap();
            if buf.trim().to_uppercase() == "EXIT" {
                break Ok(());
            }

            let Ok(timestamp) = time::SystemTime::now().duration_since(time::UNIX_EPOCH) else {
                logging!(warn, ("Невозможно получить время"));
                continue;
            };

            let parts: Vec<&str> = buf.trim().split_whitespace().collect();
            if parts.len() != 3 {
                logging!(warn, ("Неверное количество аргументов"));
                stdout
                    .write_all("Неверный формат".as_bytes())
                    .map_err(|e| e.to_string())?;
                continue;
            }

            let Ok(price): Result<f64, _> = parts[1].parse() else {
                logging!(warn, ("Неверный формат ввода price"));
                stdout
                    .write_all("еверный формат ввода цены".as_bytes())
                    .map_err(|e| e.to_string())?;
                continue;
            };

            let Ok(volume): Result<u32, _> = parts[2].parse() else {
                logging!(warn, ("Неверный формат ввода volume"));
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

            logging!(info, ("Получена котировка: {:?}", quote));

            for tx in self.subscribers.iter() {
                if let Err(_e) = tx.send(quote.clone()) {
                    logging!(warn, ("Ошибка при отправке: {}", _e));

                    stdout
                        .write_all("Ошибка при отправке".as_bytes())
                        .map_err(|e| e.to_string())?;
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
