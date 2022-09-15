use anchor_lang::prelude::*;

#[account]
pub struct Lock {
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
    pub lock: Pubkey,

    /// The key of the account that is authorized to stake into/withdraw from this [Escrow]
    pub owner: Pubkey,

    /// Bump seed
    pub bump: u8,

    /// The tokens account holding the escrow tokens
    pub tokens: Pubkey,

    /// Amount of tokens staked
    pub amount: u64,
}
