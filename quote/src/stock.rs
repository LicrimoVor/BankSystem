use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

pub type Ticker = String;

#[derive(Serialize, Deserialize, Encode, Decode, PartialEq, Debug, Clone)]
pub struct StockQuote {
    pub ticker: Ticker,
    pub price: f64,
    pub volume: u32,
    pub timestamp: u64,
}

// Методы для сериализации/десериализации
impl StockQuote {
    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.ticker, self.price, self.volume, self.timestamp
        )
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() == 4 {
            Some(StockQuote {
                ticker: parts[0].to_string(),
                price: parts[1].parse().ok()?,
                volume: parts[2].parse().ok()?,
                timestamp: parts[3].parse().ok()?,
            })
        } else {
            None
        }
    }
}

impl StockQuote {
    #[cfg(test)]
    pub fn one() -> Self {
        Self {
            ticker: "BCD".to_string(),
            price: 100.0,
            volume: 0,
            timestamp: 123123,
        }
    }

    #[cfg(test)]
    pub fn two() -> Self {
        Self {
            ticker: "PEO".to_string(),
            price: -150.0,
            volume: 523567,
            timestamp: 999999,
        }
    }
}
