use anyhow::{Context, Result};
use regex::Regex;
use sol_usd_oracle::PRICE_DECIMALS;
use solana_client::rpc_response::RpcLogsResponse;
use solana_sdk::pubkey::Pubkey;

use crate::entities::TokenCreatedLog;

pub fn to_fixed_6(txt: &str) -> Result<u64> {
    // Examples:
    // - "120" -> 120_000_000
    // - "120.12" -> 120_120_000
    // - "0.000001" -> 1
    let (real, decimal) = txt.split_once(".").unwrap_or((txt, ""));
    let real = real.parse::<u64>().context("parse real part")?;
    let decimal_part = if decimal.len() >= PRICE_DECIMALS as usize {
        let truncated = &decimal[..(PRICE_DECIMALS as usize)];
        let value = truncated.parse::<u64>().context("parse decimal part")?;
        value
    } else {
        let mut decimal = decimal.to_string();
        decimal.push_str(&"0".repeat(PRICE_DECIMALS as usize - decimal.len()));
        decimal.parse::<u64>().context("parse decimal part")?
    };
    Ok(real * 10_u64.pow(PRICE_DECIMALS as u32) + decimal_part)
}

pub fn parse_token_created(logs: &RpcLogsResponse, _program_id: Pubkey) -> Option<TokenCreatedLog> {
    let re = Regex::new(
        r"TokenCreated \{ creator: ([A-Za-z0-9]+), mint: ([A-Za-z0-9]+), decimals: (\d+), initial_supply: (\d+), fee_lamports: (\d+), sol_usd_price: (\d+), slot: (\d+) \}",
    )
    .expect("regex");

    for log in &logs.logs {
        if !log.contains("TokenCreated") {
            continue;
        }
        if let Some(caps) = re.captures(log) {
            let creator = caps.get(1)?.as_str().to_string();
            let mint = caps.get(2)?.as_str().to_string();
            let decimals = caps.get(3)?.as_str().parse().ok()?;
            let initial_supply = caps.get(4)?.as_str().parse().ok()?;
            let fee_lamports = caps.get(5)?.as_str().parse().ok()?;
            let sol_usd_price = caps.get(6)?.as_str().parse().ok()?;
            let slot = caps.get(7)?.as_str().parse().ok()?;

            return Some(TokenCreatedLog {
                creator,
                mint,
                decimals,
                initial_supply,
                fee_lamports,
                sol_usd_price,
                slot,
                signature: logs.signature.clone(),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_client::rpc_response::RpcLogsResponse;

    #[test]
    fn to_fixed_6_parses_integer_and_fractional_part() {
        assert_eq!(to_fixed_6("120").unwrap(), 120_000_000);
        assert_eq!(to_fixed_6("120.12").unwrap(), 120_120_000);
        assert_eq!(to_fixed_6("0.000001").unwrap(), 1);
    }

    #[test]
    fn to_fixed_6_truncates_fraction_to_six_digits() {
        assert_eq!(to_fixed_6("1.1234569").unwrap(), 1_123_456);
    }

    #[test]
    fn to_fixed_6_rejects_invalid_input() {
        assert!(to_fixed_6("abc").is_err());
    }

    #[test]
    fn parse_token_created_reads_expected_fields() {
        let logs = RpcLogsResponse {
            signature: "5Yf8k3w2J3k9R8B9Q2".to_string(),
            err: None,
            logs: vec![
                "Program xyz log".to_string(),
                "Program log: TokenCreated { creator: 4N8wYzU2aB3cD4eF5gH6iJ7kL8mN9pQ1R2sT3uV4wXy, mint: 7K9mP2xQ8dW1vR6nT4cB3zY5aL7fG2hJ9sD1qW8eR4t, decimals: 6, initial_supply: 1000000, fee_lamports: 41666666, sol_usd_price: 120000000, slot: 77 }".to_string(),
            ],
        };

        let parsed = parse_token_created(&logs, Pubkey::new_unique()).expect("event should parse");
        assert_eq!(parsed.decimals, 6);
        assert_eq!(parsed.initial_supply, 1_000_000);
        assert_eq!(parsed.fee_lamports, 41_666_666);
        assert_eq!(parsed.sol_usd_price, 120_000_000);
        assert_eq!(parsed.slot, 77);
        assert_eq!(parsed.signature, logs.signature);
    }

    #[test]
    fn parse_token_created_returns_none_for_unrelated_logs() {
        let logs = RpcLogsResponse {
            signature: "4m4u3z".to_string(),
            err: None,
            logs: vec!["Program log: some other event".to_string()],
        };
        assert!(parse_token_created(&logs, Pubkey::new_unique()).is_none());
    }
}
