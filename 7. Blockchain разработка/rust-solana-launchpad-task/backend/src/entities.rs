use crate::{config::Config, utils::to_fixed_6};
use anyhow::Result;
use serde::Serialize;

#[derive(Clone)]
pub enum PriceSource {
    Mock(u64),
    Http { url: String },
}

impl PriceSource {
    pub fn from_config(cfg: &Config) -> Self {
        if let Some(mock) = cfg.mock_price {
            PriceSource::Mock(mock)
        } else if let Some(url) = cfg.price_api_url.clone() {
            PriceSource::Http { url }
        } else {
            PriceSource::Http {
                url: "https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT".to_string(),
            }
        }
    }

    pub async fn fetch_price(&self) -> Result<u64> {
        match self {
            PriceSource::Mock(val) => Ok(*val),
            PriceSource::Http { url } => {
                #[derive(serde::Deserialize)]
                struct Resp {
                    price: String,
                }
                let resp: Resp = reqwest::get(url).await?.json().await?;
                to_fixed_6(&resp.price)
            }
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TokenCreatedLog {
    pub creator: String,
    pub mint: String,
    pub decimals: u8,
    pub initial_supply: u64,
    pub fee_lamports: u64,
    pub sol_usd_price: u64,
    pub slot: u64,
    pub signature: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;
    use std::time::Duration;

    fn sample_cfg(mock_price: Option<u64>, price_api_url: Option<&str>) -> Config {
        Config {
            rpc_http: "http://127.0.0.1:8899".to_string(),
            rpc_ws: "ws://127.0.0.1:8900".to_string(),
            oracle_program_id: Pubkey::new_unique(),
            oracle_state: Pubkey::new_unique(),
            minter_program_id: Pubkey::new_unique(),
            backend_keypair_path: "/tmp/id.json".to_string(),
            price_poll_interval: Duration::from_secs(60),
            mock_price,
            price_api_url: price_api_url.map(ToString::to_string),
        }
    }

    #[test]
    fn price_source_prefers_mock_over_url() {
        let cfg = sample_cfg(Some(123), Some("https://example.com/price"));
        let source = PriceSource::from_config(&cfg);
        match source {
            PriceSource::Mock(v) => assert_eq!(v, 123),
            PriceSource::Http { .. } => panic!("expected mock source"),
        }
    }

    #[test]
    fn price_source_uses_default_url_when_no_override() {
        let cfg = sample_cfg(None, None);
        let source = PriceSource::from_config(&cfg);
        match source {
            PriceSource::Mock(_) => panic!("expected http source"),
            PriceSource::Http { url } => {
                assert_eq!(
                    url,
                    "https://api.binance.com/api/v3/ticker/price?symbol=SOLUSDT"
                )
            }
        }
    }
}
