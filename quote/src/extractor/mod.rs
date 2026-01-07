mod api;
mod console;
mod file;
mod random;
use crate::types::stock::StockQuote;
use clap::ValueEnum;
pub use console::ConsoleExtractor;
pub use file::FileMockExtractor;
pub use random::RandomExtractor;
use std::sync::mpsc::Receiver;

const SLEEP_TIME: std::time::Duration = std::time::Duration::from_millis(1_000);

/// Трейт экстрактора
pub trait Extractor: Send {
    fn run(self: Box<Self>) -> Result<(), String>;
    fn subscribe(&mut self) -> Receiver<StockQuote>;
}

/// ## Тип Extractor
#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum ExtractorType {
    Console,
    File,
    Api,
    Random,
}
