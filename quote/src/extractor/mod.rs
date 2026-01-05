mod console;
mod random;
use crate::types::stock::{StockQuote, Ticker};
use std::sync::mpsc::Receiver;

pub use console::ConsoleExtractor;
pub use random::RandomExtractor;

const SLEEP_TIME: std::time::Duration = std::time::Duration::from_millis(1000);

/// Трейт экстрактора
pub trait Extractor {
    fn new() -> Self;
    fn run(self) -> Result<Self, String>
    where
        Self: Sized;
    fn subscribe(&mut self) -> Receiver<StockQuote>;
}
