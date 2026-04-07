use anyhow::Result;
use solana_client::rpc_client;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    instruction,
    message::{AccountMeta, Message},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction as system_instruction;

const PROGRAM_ID: &'static str = "8aWuRqDF3Z9cEGhHQC6EoY6bDuG6qJUGyNRgbCa7vyuT";

fn main() -> Result<()> {
    let client = rpc_client::RpcClient::new_with_commitment(
        "http://127.0.0.1:8899".to_string(),
        CommitmentConfig::processed(),
    );

    // --- Создаем дефолт аккаунт и пополняем его ---
    let payer = Keypair::new();
    println!("payer: {}", payer.pubkey());
    let sig = client.request_airdrop(&payer.pubkey(), 2 * LAMPORTS_PER_SOL)?;

    loop {
        if client.confirm_transaction(&sig)? {
            break;
        }
    }

    println!("balance: {}", client.get_balance(&payer.pubkey())?);

    // --- Создаем дата аккаунт ---
    let program_id: Pubkey = PROGRAM_ID.parse()?;
    let rent = client.get_minimum_balance_for_rent_exemption(8)?;
    let data_payer = Keypair::new();
    let inst = system_instruction::create_account(
        &payer.pubkey(),
        &data_payer.pubkey(),
        rent,
        8,
        &program_id,
    );

    let last_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[inst],
        Some(&payer.pubkey()),
        &[&payer, &data_payer], // ОБА подписывают
        last_blockhash,
    );

    let sig = client.send_and_confirm_transaction(&tx)?;

    loop {
        if client.confirm_transaction(&sig)? {
            break;
        }
    }

    println!("data payer: {}", data_payer.pubkey());

    // --- Инициализируем дата аккаунт ---
    let inst = instruction::Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(data_payer.pubkey(), false),
            AccountMeta::new(payer.pubkey(), true),
        ],
        data: vec![0],
    };

    let msg = Message::new(&[inst], Some(&payer.pubkey()));
    let last_blockhash = client.get_latest_blockhash()?;
    let tx = Transaction::new(&[payer.insecure_clone()], msg, last_blockhash);
    let sig = client.send_and_confirm_transaction(&tx)?;

    loop {
        if client.confirm_transaction(&sig)? {
            break;
        }
    }

    println!("Signature: {}", sig);

    // --- Увеличиваем счетчик ---
    let inst = instruction::Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(data_payer.pubkey(), false),
            AccountMeta::new(payer.pubkey(), true),
        ],
        data: vec![1],
    };
    let msg = Message::new(&[inst], Some(&payer.pubkey()));
    for i in 0..10 {
        let last_blockhash = client.get_latest_blockhash()?;
        let tx = Transaction::new(&[payer.insecure_clone()], msg.clone(), last_blockhash);
        let sig = client.send_and_confirm_transaction(&tx)?;

        println!("{i} Incremented: {sig}");
    }

    let data = client.get_account(&data_payer.pubkey())?;
    println!("Data: {:#?}", data);
    Ok(())
}
