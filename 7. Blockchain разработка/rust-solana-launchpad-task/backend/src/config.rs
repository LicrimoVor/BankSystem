use anyhow::{Context, Result};
use solana_sdk::pubkey::Pubkey;
use std::{env, str::FromStr, time::Duration};

pub const DEFAULT_PRICE_POLL_INTERVAL_SEC: u64 = 600; // 10 minutes; live price from Binance when MOCK_PRICE is not set

#[derive(Clone)]
pub struct Config {
    pub rpc_http: String,
    pub rpc_ws: String,
    pub oracle_program_id: Pubkey,
    pub oracle_state: Pubkey,
    pub minter_program_id: Pubkey,
    pub backend_keypair_path: String,
    pub price_poll_interval: Duration,
    pub mock_price: Option<u64>,
    pub price_api_url: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let rpc_http =
            env::var("SOLANA_RPC_HTTP").context("SOLANA_RPC_HTTP env var is required")?;
        let rpc_ws = env::var("SOLANA_RPC_WS").context("SOLANA_RPC_WS env var is required")?;
        let oracle_program_id = Pubkey::from_str(
            &env::var("ORACLE_PROGRAM_ID").context("ORACLE_PROGRAM_ID is required")?,
        )?;
        let oracle_state =
            Pubkey::from_str(&env::var("ORACLE_STATE_PUBKEY").context("ORACLE_STATE_PUBKEY")?)?;
        let minter_program_id = Pubkey::from_str(
            &env::var("MINTER_PROGRAM_ID").context("MINTER_PROGRAM_ID is required")?,
        )?;
        let mut backend_keypair_path =
            env::var("BACKEND_KEYPAIR_PATH").context("BACKEND_KEYPAIR_PATH is required")?;
        if backend_keypair_path.starts_with("~/") {
            if let Some(home) = env::var_os("HOME") {
                backend_keypair_path =
                    format!("{}/{}", home.to_string_lossy(), &backend_keypair_path[2..]);
            }
        }
        let poll = env::var("PRICE_POLL_INTERVAL_SEC")
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(DEFAULT_PRICE_POLL_INTERVAL_SEC);
        let mock_price = env::var("MOCK_PRICE")
            .ok()
            .and_then(|s| s.parse::<u64>().ok());
        let price_api_url = env::var("PRICE_API_URL").ok();

        Ok(Self {
            rpc_http,
            rpc_ws,
            oracle_program_id,
            oracle_state,
            minter_program_id,
            backend_keypair_path,
            price_poll_interval: Duration::from_secs(poll),
            mock_price,
            price_api_url,
        })
    }
}
