use borsh::BorshDeserialize;
use solana_program::{pubkey::Pubkey, system_program};
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{Keypair, Signer},
    transaction::Transaction,
};

use vaultboard::{instruction::VaultInstruction, state::VaultState};

fn program_id() -> Pubkey {
    Pubkey::new_from_array([7u8; 32])
}

fn ix_initialize(pid: Pubkey, owner: Pubkey, vault: Pubkey, message: &str) -> Instruction {
    Instruction {
        program_id: pid,
        accounts: vec![
            AccountMeta::new(owner, true),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: borsh::to_vec(&VaultInstruction::Initialize {
            message: message.to_string(),
        })
        .unwrap(),
    }
}

fn ix_deposit(pid: Pubkey, owner: Pubkey, vault: Pubkey, lamports: u64) -> Instruction {
    Instruction {
        program_id: pid,
        accounts: vec![
            AccountMeta::new(owner, true),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: borsh::to_vec(&VaultInstruction::Deposit { lamports }).unwrap(),
    }
}

fn ix_update(pid: Pubkey, owner: Pubkey, vault: Pubkey, message: &str) -> Instruction {
    Instruction {
        program_id: pid,
        accounts: vec![
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(vault, false),
        ],
        data: borsh::to_vec(&VaultInstruction::UpdateMessage {
            message: message.to_string(),
        })
        .unwrap(),
    }
}

fn ix_withdraw(pid: Pubkey, owner: Pubkey, vault: Pubkey, lamports: u64) -> Instruction {
    Instruction {
        program_id: pid,
        accounts: vec![
            AccountMeta::new(owner, true),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: borsh::to_vec(&VaultInstruction::Withdraw { lamports }).unwrap(),
    }
}

#[tokio::test]
async fn e2e_vaultboard_workflow() {
    let pid = program_id();

    let mut pt = ProgramTest::new(
        "vaultboard",
        pid,
        processor!(vaultboard::entrypoint::process_instruction),
    );

    let (mut banks_client, payer, recent_blockhash) = pt.start().await;

    let user = Keypair::new();

    // Дадим пользователю SOL
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[solana_sdk::system_instruction::transfer(
                &payer.pubkey(),
                &user.pubkey(),
                2_000_000_000,
            )],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        ))
        .await
        .unwrap();

    let (vault_pda, _bump) = Pubkey::find_program_address(&[b"vault", user.pubkey().as_ref()], &pid);

    // 1) Initialize
    let bh = banks_client.get_latest_blockhash().await.unwrap();
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[ix_initialize(pid, user.pubkey(), vault_pda, "hello")],
            Some(&user.pubkey()),
            &[&user],
            bh,
        ))
        .await
        .unwrap();

    let vault_acc = banks_client.get_account(vault_pda).await.unwrap().unwrap();
    assert_eq!(vault_acc.owner, pid);
    assert_eq!(vault_acc.data.len(), VaultState::LEN);

    let state = VaultState::try_from_slice(&vault_acc.data).unwrap();
    assert!(state.is_initialized);
    assert_eq!(state.owner, user.pubkey());
    assert_eq!(state.message_len, 5);
    assert_eq!(&state.message[..5], b"hello");

    // 2) Deposit 0.1 SOL
    let before_vault_lamports = vault_acc.lamports;
    let bh = banks_client.get_latest_blockhash().await.unwrap();
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[ix_deposit(pid, user.pubkey(), vault_pda, 100_000_000)],
            Some(&user.pubkey()),
            &[&user],
            bh,
        ))
        .await
        .unwrap();

    let vault_acc2 = banks_client.get_account(vault_pda).await.unwrap().unwrap();
    assert_eq!(vault_acc2.lamports, before_vault_lamports + 100_000_000);
    let state2 = VaultState::try_from_slice(&vault_acc2.data).unwrap();
    assert_eq!(state2.total_deposited, 100_000_000);

    // 3) UpdateMessage
    let bh = banks_client.get_latest_blockhash().await.unwrap();
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[ix_update(pid, user.pubkey(), vault_pda, "updated")],
            Some(&user.pubkey()),
            &[&user],
            bh,
        ))
        .await
        .unwrap();

    let vault_acc3 = banks_client.get_account(vault_pda).await.unwrap().unwrap();
    let state3 = VaultState::try_from_slice(&vault_acc3.data).unwrap();
    assert_eq!(state3.message_len, 7);
    assert_eq!(&state3.message[..7], b"updated");
    assert!(state3.message[7..].iter().all(|&x| x == 0));

    // 4) Withdraw 0.05 SOL
    let before_vault_lamports = vault_acc3.lamports;
    let bh = banks_client.get_latest_blockhash().await.unwrap();
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[ix_withdraw(pid, user.pubkey(), vault_pda, 50_000_000)],
            Some(&user.pubkey()),
            &[&user],
            bh,
        ))
        .await
        .unwrap();

    let vault_acc4 = banks_client.get_account(vault_pda).await.unwrap().unwrap();
    assert_eq!(vault_acc4.lamports, before_vault_lamports - 50_000_000);
}

#[tokio::test]
async fn test_error_cases() {
    let pid = program_id();
    let mut pt = ProgramTest::new(
        "vaultboard",
        pid,
        processor!(vaultboard::entrypoint::process_instruction),
    );
    let (mut banks_client, payer, recent_blockhash) = pt.start().await;

    let user = Keypair::new();
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[solana_sdk::system_instruction::transfer(
                &payer.pubkey(),
                &user.pubkey(),
                2_000_000_000,
            )],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        ))
        .await
        .unwrap();

    let (vault_pda, _bump) = Pubkey::find_program_address(&[b"vault", user.pubkey().as_ref()], &pid);

    // (1) Initialize с message длиной 100 => ошибка
    let long_msg = "a".repeat(100);
    let bh = banks_client.get_latest_blockhash().await.unwrap();
    let res = banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[ix_initialize(pid, user.pubkey(), vault_pda, &long_msg)],
            Some(&user.pubkey()),
            &[&user],
            bh,
        ))
        .await;
    assert!(res.is_err());

    // Нормально инициализируем
    let bh = banks_client.get_latest_blockhash().await.unwrap();
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[ix_initialize(pid, user.pubkey(), vault_pda, "ok")],
            Some(&user.pubkey()),
            &[&user],
            bh,
        ))
        .await
        .unwrap();

    // (2) Deposit lamports=0 => ошибка
    let bh = banks_client.get_latest_blockhash().await.unwrap();
    let res = banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[ix_deposit(pid, user.pubkey(), vault_pda, 0)],
            Some(&user.pubkey()),
            &[&user],
            bh,
        ))
        .await;
    assert!(res.is_err());

    // (3) UpdateMessage без подписи => ошибка (owner not signer)
    let ix = Instruction {
        program_id: pid,
        accounts: vec![
            AccountMeta::new_readonly(user.pubkey(), false), // <- not signer
            AccountMeta::new(vault_pda, false),
        ],
        data: borsh::to_vec(&VaultInstruction::UpdateMessage {
            message: "x".to_string(),
        })
        .unwrap(),
    };
    let bh = banks_client.get_latest_blockhash().await.unwrap();
    let res = banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[ix],
            Some(&user.pubkey()),
            &[&user],
            bh,
        ))
        .await;
    assert!(res.is_err());

    // (4) Подмена PDA: withdraw с неправильным vault => ошибка
    let fake_vault = Pubkey::new_unique();
    let bh = banks_client.get_latest_blockhash().await.unwrap();
    let res = banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[ix_withdraw(pid, user.pubkey(), fake_vault, 1)],
            Some(&user.pubkey()),
            &[&user],
            bh,
        ))
        .await;
    assert!(res.is_err());
}
