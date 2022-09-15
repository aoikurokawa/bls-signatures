use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;

use crate::*;

#[account]
pub struct Locker {
    /// Base account used to generate signer seeds
    pub base: Pubkey,
    /// Bump seed
    pub bump: u8,
    /// Mint of the token that must be locked in the [Lock]
    pub token_mint: Pubkey,
}

#[account]
pub struct Escrow {
    /// The [Lock] that this [Escrow] is part of
    pub locker: Pubkey,

    /// The key of the account that is authorized to stake into/withdraw from this [Escrow]
    pub owner: Pubkey,

    /// Bump seed
    pub bump: u8,

    /// The tokens account holding the escrow tokens
    pub tokens: Pubkey,

    /// Amount of tokens staked
    pub amount: u64,

    /// When the [Escrow::owner] started their escrow
    pub escrow_started_at: i64,

    /// When the escrow unlocks; i.e. the [Escrow::owner] is scheduled to be allowed to withdraw their tokens
    pub escrow_ends_at: i64,
}

impl Escrow {
    pub const LEN: usize = PUBKEY_BYTES + PUBKEY_BYTES + 1 + PUBKEY_BYTES + 8 + 8 + 8;

    pub fn record_lock_event(
        &mut self,
        next_escrow_started_at: i64,
        next_escrow_ends_at: i64,
    ) -> Result<()> {
        self.escrow_started_at = next_escrow_started_at;
        self.escrow_ends_at = next_escrow_ends_at;
        Ok(())
    }
}
