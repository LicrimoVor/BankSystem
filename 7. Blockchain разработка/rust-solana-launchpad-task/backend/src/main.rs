pub mod config;
pub mod entities;
pub mod utils;
use crate::config::Config;
use crate::entities::PriceSource;
use crate::utils::parse_token_created;
use anchor_lang::{InstructionData, ToAccountMetas};
use anyhow::{anyhow, Context, Result};
use dotenvy::dotenv;
use futures::StreamExt;
use sol_usd_oracle::{accounts, instruction};
use solana_client::nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient};
use solana_rpc_client_types::config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    signature::{read_keypair_file, Keypair, Signature, Signer},
    transaction::Transaction,
};
use std::sync::Arc;
use tokio::time::interval;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cfg = Config::from_env()?;
    let price_source = PriceSource::from_config(&cfg);
    let admin = Arc::new(
        read_keypair_file(&cfg.backend_keypair_path)
            .map_err(|e| anyhow!(e.to_string()))
            .context("read backend keypair")?,
    );

    let price_task = tokio::spawn(run_price_updater(cfg.clone(), price_source, admin.clone()));
    let listener_task = tokio::spawn(run_event_listener(cfg.clone()));

    let (price_res, listener_res) = tokio::try_join!(price_task, listener_task)?;
    price_res?;
    listener_res?;
    Ok(())
}

async fn run_price_updater(
    cfg: Config,
    price_source: PriceSource,
    admin: Arc<Keypair>,
) -> Result<()> {
    let client = RpcClient::new(cfg.rpc_http.clone());
    let mut ticker = interval(cfg.price_poll_interval);
    info!("Starting price updater");

    // Run one update immediately on startup
    try_update_price(&client, &cfg, &price_source, admin.clone(), "initial").await;

    loop {
        ticker.tick().await;
        try_update_price(&client, &cfg, &price_source, admin.clone(), "scheduled").await;
    }
}

async fn try_update_price(
    client: &RpcClient,
    cfg: &Config,
    price_source: &PriceSource,
    admin: Arc<Keypair>,
    kind: &'static str,
) {
    match price_source.fetch_price().await {
        Ok(price) => {
            if price == 0 {
                warn!(
                    "Skipped {} price update because fetched price is zero",
                    kind
                );
                return;
            }
            match submit_price(client, cfg, price, admin).await {
                Ok(sig) => info!(%sig, price, "oracle price updated ({})", kind),
                Err(err) => error!(?err, "failed to submit {} price", kind),
            }
        }
        Err(err) => error!(?err, "failed to fetch {} price", kind),
    }
}

async fn submit_price(
    client: &RpcClient,
    cfg: &Config,
    new_price: u64,
    admin: Arc<Keypair>,
) -> Result<Signature> {
    info!(new_price, "submitting price update");
    let ix_data = instruction::UpdatePrice { new_price }.data();
    let accounts = accounts::UpdatePrice {
        oracle: cfg.oracle_state,
        admin: admin.pubkey(),
    }
    .to_account_metas(None);

    let ix = Instruction {
        program_id: cfg.oracle_program_id,
        accounts,
        data: ix_data,
    };

    let bh = client
        .get_latest_blockhash()
        .await
        .context("fetch blockhash")?;

    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&admin.pubkey()), &[admin.as_ref()], bh);

    let sig = client
        .send_and_confirm_transaction_with_spinner_and_commitment(
            &tx,
            CommitmentConfig::confirmed(),
        )
        .await
        .context("send price tx")?;

    Ok(sig)
}

async fn run_event_listener(cfg: Config) -> Result<()> {
    let client = PubsubClient::new(&cfg.rpc_ws)
        .await
        .context("connect pubsub ws")?;
    info!("Starting event listener");

    let (mut stream, _unsub) = client
        .logs_subscribe(
            RpcTransactionLogsFilter::Mentions(vec![cfg.minter_program_id.to_string()]),
            RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::confirmed()),
            },
        )
        .await
        .context("subscribe to logs")?;

    while let Some(value) = stream.next().await {
        if let Some(parsed) = parse_token_created(&value.value, cfg.minter_program_id) {
            info!(
                target: "token_created",
                "creator={} mint={} decimals={} supply={} fee_lamports={} price={} slot={} sig={}",
                parsed.creator,
                parsed.mint,
                parsed.decimals,
                parsed.initial_supply,
                parsed.fee_lamports,
                parsed.sol_usd_price,
                parsed.slot,
                parsed.signature
            );
            if let Ok(json) = serde_json::to_string(&parsed) {
                println!("{json}");
            }
        }
    }
    Ok(())
}
