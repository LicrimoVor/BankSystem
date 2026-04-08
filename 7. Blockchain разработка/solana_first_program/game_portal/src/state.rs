use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Состояние двери (хранится в PDA-аккаунте Portal).
/// Держим всё фиксированного размера и простого формата.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct PortalState {
    /// Чтобы отличать "пустой" аккаунт от инициализированного.
    pub is_initialized: bool,

    /// Владелец двери (обычно игрок).
    pub owner: Pubkey,

    /// Какая программа Lever разрешена для управления этой дверью.
    pub lever_program: Pubkey,

    /// bump PDA двери (чтобы позже можно было проверять/подписывать при необходимости).
    pub bump: u8,

    /// 0 = закрыта, 1 = открыта.
    pub is_open: u8,
}

impl PortalState {
    /// Фиксированный размер аккаунта под состояние.
    pub const LEN: usize = 1 + 32 + 32 + 1 + 1;
}
