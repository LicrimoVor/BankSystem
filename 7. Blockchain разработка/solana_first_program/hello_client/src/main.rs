use std::{thread::sleep, time::Duration};

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_rpc_client_api::config::RpcTransactionConfig;
use solana_sdk::{
    config::program,
    instruction::{AccountMeta, Instruction},
    message::Message,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_transaction_status::UiTransactionEncoding;

const PROGRAM_ID: &str = "GkPjeZSqqbFC36DcCD3JgJwQMjAdAUw1KXkscde81ayx";

fn main() -> Result<()> {
    let client = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    let payer = Keypair::new();
    let ix = Instruction {
        program_id: "11111111111111111111111111111111".parse().unwrap(),
        accounts: vec![],
        data: vec![],
    };

    let airdrop_sig = client.request_airdrop(&payer.pubkey(), 1 * LAMPORTS_PER_SOL)?;
    println!("Airdrop initiated. Waiting for confirmation...");

    sleep(Duration::from_secs(4));

    let confirmed =
        client.confirm_transaction_with_commitment(&airdrop_sig, CommitmentConfig::confirmed())?;

    if !confirmed.value {
        anyhow::bail!("Airdrop transaction not confirmed");
    }

    println!("Airdrop confirmed!");

    let data: Vec<u8> = vec![42];

    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true), // writable + signer
    ];

    let program_id: Pubkey = PROGRAM_ID.parse()?;

    let ix = Instruction {
        program_id,
        accounts,
        data,
    };

    let message = Message::new(&[ix], Some(&payer.pubkey()));

    println!("--- message.account_keys ---");
    for (i, key) in message.account_keys.iter().enumerate() {
        println!("index {:>2}: {}", i, key);
    }

    let mut tx = Transaction::new_unsigned(message);

    let recent_blockhash = client.get_latest_blockhash()?;
    tx.sign(&[&payer], recent_blockhash);

    let signature = client.send_and_confirm_transaction(&tx)?;
    println!("Program call signature: {}", signature);

    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: None,
    };

    let tx_info = client.get_transaction_with_config(&signature, config)?;

    if let Some(meta) = tx_info.transaction.meta {
        println!("--- program logs from transaction meta ---");
        use solana_transaction_status::option_serializer::OptionSerializer;
        if let OptionSerializer::Some(logs) = meta.log_messages {
            for line in logs {
                println!("{}", line);
            }
        } else {
            println!("No log_messages in meta");
        }
    }

    Ok(())
}
