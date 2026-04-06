use std::{thread::sleep, time};

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction as system_instruction;

fn main() -> Result<()> {
    // 1) Подключаемся к локальному RPC
    let rpc_url = "http://localhost:8899".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // 2) Генерируем ключи (в реальных приложениях ключи обычно загружают из файла/кошелька)
    let sender = Keypair::new();
    let treasury = Keypair::new();

    println!("Sender pubkey:   {}", sender.pubkey());
    println!("Treasury pubkey: {}", treasury.pubkey());

    // 3) Делаем airdrop на sender, чтобы были деньги на перевод и комиссию
    let airdrop_sig = client.request_airdrop(&sender.pubkey(), 2 * LAMPORTS_PER_SOL)?;
    client.confirm_transaction(&airdrop_sig)?;
    println!("Airdrop signature: {}", airdrop_sig);

    // не успевает обработать транзакцию, поэтому ждем
    sleep(time::Duration::from_secs(4));

    // 4) Балансы до
    let sender_before = client.get_balance(&sender.pubkey())?;
    let treasury_before = client.get_balance(&treasury.pubkey())?;
    println!(
        "Balances before: sender={} lamports, treasury={} lamports",
        sender_before, treasury_before
    );

    // 5) Собираем instruction: System Program transfer
    // Это "операция", но ещё не транзакция.
    let amount = (0.1_f64 * LAMPORTS_PER_SOL as f64) as u64;
    let transfer_ix = system_instruction::transfer(&sender.pubkey(), &treasury.pubkey(), amount);

    // 6) Собираем transaction: контейнер инструкций + подписи
    let latest_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[transfer_ix],         // инструкции
        Some(&sender.pubkey()), // fee payer (платит комиссию)
        &[&sender],             // подписанты
        latest_blockhash,       // свежий blockhash
    );

    // 7) Отправляем и подтверждаем
    let sig = client.send_and_confirm_transaction(&tx)?;
    println!("Transfer transaction signature: {}", sig);

    // 8) Балансы после
    let sender_after = client.get_balance(&sender.pubkey())?;
    let treasury_after = client.get_balance(&treasury.pubkey())?;
    println!(
        "Balances after:  sender={} lamports, treasury={} lamports",
        sender_after, treasury_after
    );

    // 9) Мини-проверка результата:
    // treasury должен увеличиться минимум на amount (точно на amount, так как комиссию платит sender).
    if treasury_after < treasury_before + amount {
        anyhow::bail!("Treasury balance did not increase as expected");
    }

    Ok(())
}
