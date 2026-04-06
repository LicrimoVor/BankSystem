use std::{thread::sleep, time::Duration};

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_rpc_client_api::config::RpcTransactionConfig;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_transaction_status::UiTransactionEncoding;

fn main() -> Result<()> {
    // 1) RPC клиент localnet
    let client = RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::confirmed(),
    );

    // 2) Плательщик комиссии (payer) — это локальный ключ в коде
    // В реальных приложениях payer часто берут из файла wallet’а или аппаратного кошелька.
    let payer = Keypair::new();

    // Дадим payer немного SOL через airdrop, иначе он не сможет платить fee
    let airdrop_sig = client.request_airdrop(&payer.pubkey(), 1_000_000_000)?;
    println!("Airdrop initiated. Waiting for confirmation...");

    sleep(Duration::from_secs(4));

    // Ждём подтверждения airdrop, с тайм-аутом
    let confirmed =
        client.confirm_transaction_with_commitment(&airdrop_sig, CommitmentConfig::confirmed())?;

    if !confirmed.value {
        anyhow::bail!("Airdrop transaction not confirmed");
    }

    println!("Airdrop confirmed!");

    // 3) Program id задеплоенной программы
    let program_id: Pubkey = "GkPjeZSqqbFC36DcCD3JgJwQMjAdAUw1KXkscde81ayx".parse()?;

    // 4) Данные инструкции (любой байтовый payload для демонстрации)
    let data: Vec<u8> = vec![42];

    // 5) Аккаунты, которые мы явно передаём программе.
    // Для демонстрации передадим payer как writable signer.
    // Программа может прочитать, что ей передали 1 аккаунт, и увидеть pubkey.
    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true), // writable + signer
    ];

    // 6) Instruction: "вызови program_id, дай ей accounts и data"
    let ix = Instruction {
        program_id,
        accounts,
        data,
    };

    // 7) Message: собирает инструкции и формирует account_keys.
    // Второй аргумент — payer: тот, кто платит fee.
    let message = Message::new(&[ix], Some(&payer.pubkey()));

    // 8) Покажем account_keys и объясним, что это “явный список аккаунтов”
    println!("--- message.account_keys ---");
    for (i, key) in message.account_keys.iter().enumerate() {
        println!("index {:>2}: {}", i, key);
    }

    // 9) Transaction: создаём неподписанную транзакцию из message
    let mut tx = Transaction::new_unsigned(message);

    // 10) Подписываем транзакцию.
    // Подписывается message + recent blockhash.
    let recent_blockhash = client.get_latest_blockhash()?;
    tx.sign(&[&payer], recent_blockhash);

    // 11) Отправка и подтверждение
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
