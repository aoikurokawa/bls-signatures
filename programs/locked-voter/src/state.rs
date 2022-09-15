use anchor_lang::prelude::*;

#[account]
pub struct Lock {
    base: Pubkey,
}
