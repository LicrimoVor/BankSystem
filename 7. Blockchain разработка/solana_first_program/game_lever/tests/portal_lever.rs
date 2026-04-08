use borsh::BorshDeserialize;
use game_portal::state::PortalState;
use solana_program::pubkey::Pubkey;
use solana_program_test::{ProgramTest, processor};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_system_interface::instruction as system_instruction;

#[tokio::test]
async fn portal_cannot_be_opened_directly_only_via_lever() {
    // Фиксируем program_id для теста (чтобы PDA были стабильными).
    let portal_id = Pubkey::new_from_array([1u8; 32]);
    let lever_id = Pubkey::new_from_array([2u8; 32]);
    // Регистрируем обе программы в program-test.
    let mut pt = ProgramTest::new(
        "portal",
        portal_id,
        processor!(portal::entrypoint::process_instruction),
    );
    pt.add_program(
        "lever",
        lever_id,
        processor!(lever::entrypoint::process_instruction),
    );

    // Запускаем тестовую "локальную сеть".
    let (mut banks_client, payer, recent_blockhash) = pt.start().await;

    // Игрок (owner двери).
    let player = Keypair::new();

    // Дадим игроку SOL, чтобы он мог оплачивать create_account.
    let player_lamports = 2 * LAMPORTS_PER_SOL;
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[solana_sdk::system_instruction::transfer(
                &payer.pubkey(),
                &player.pubkey(),
                player_lamports,
            )],
            Some(&payer.pubkey()),
            &[&payer],
            recent_blockhash,
        ))
        .await
        .unwrap();

    // Вычисляем PDA двери: ["portal", player]
    let (portal_pda, _portal_bump) =
        Pubkey::find_program_address(&[b"portal", player.pubkey().as_ref()], &portal_id);

    // Вычисляем PDA рычага: ["authority"] для lever program
    let (lever_authority, _auth_bump) = Pubkey::find_program_address(&[b"authority"], &lever_id);

    // --- 1) Initialize двери ---
    // Portal::Initialize tag = 0
    let init_ix = Instruction {
        program_id: portal_id,
        accounts: vec![
            AccountMeta::new(player.pubkey(), true),    // owner payer
            AccountMeta::new(portal_pda, false),        // portal PDA (создастся)
            AccountMeta::new_readonly(lever_id, false), // lever program id (как аккаунт)
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: vec![0],
    };

    let bh = banks_client.get_latest_blockhash().await.unwrap();
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[init_ix],
            Some(&player.pubkey()),
            &[&player],
            bh,
        ))
        .await
        .unwrap();

    // --- 2) Пытаемся открыть дверь напрямую (читерство) ---
    // Portal::Open tag = 1
    // Важно: мы НЕ можем сделать lever_authority signer в прямом вызове.
    // Поэтому Portal должен отклонить вызов.
    let direct_open_ix = Instruction {
        program_id: portal_id,
        accounts: vec![
            AccountMeta::new_readonly(player.pubkey(), true),
            AccountMeta::new(portal_pda, false),
            AccountMeta::new_readonly(lever_authority, false), // не signer
            AccountMeta::new_readonly(lever_id, false),
        ],
        data: vec![1],
    };

    let bh = banks_client.get_latest_blockhash().await.unwrap();
    let direct_res = banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[direct_open_ix],
            Some(&player.pubkey()),
            &[&player],
            bh,
        ))
        .await;

    assert!(direct_res.is_err(), "Direct open must fail");

    // --- 3) Создаём lever_authority (через Lever::InitAuthority) ---
    let init_auth_ix = Instruction {
        program_id: lever_id,
        accounts: vec![
            AccountMeta::new(player.pubkey(), true),  // payer
            AccountMeta::new(lever_authority, false), // PDA создастся
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: vec![0], // Lever::InitAuthority
    };

    let bh = banks_client.get_latest_blockhash().await.unwrap();
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[init_auth_ix],
            Some(&player.pubkey()),
            &[&player],
            bh,
        ))
        .await
        .unwrap();

    // --- 4) Нажимаем рычаг: Lever::PressToOpen => CPI в Portal::Open ---
    let press_open_ix = Instruction {
        program_id: lever_id,
        accounts: vec![
            AccountMeta::new_readonly(player.pubkey(), true),
            AccountMeta::new(portal_pda, false), // writable не нужно на meta? лучше new()
            AccountMeta::new_readonly(portal_id, false), // portal program account
            AccountMeta::new_readonly(lever_authority, false),
            AccountMeta::new_readonly(lever_id, false), // lever program account (сам себя)
        ],
        data: vec![1], // Lever::PressToOpen
    };

    let bh = banks_client.get_latest_blockhash().await.unwrap();
    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[press_open_ix],
            Some(&player.pubkey()),
            &[&player],
            bh,
        ))
        .await
        .unwrap();
    // Проверяем, что дверь реально открылась (is_open=1).
    let portal_acc = banks_client.get_account(portal_pda).await.unwrap().unwrap();
    let state = PortalState::try_from_slice(&portal_acc.data).unwrap();
    assert_eq!(state.is_open, 1);
}
