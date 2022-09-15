use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

mod account_structs;
mod state;

pub use account_structs::*;
pub use state::*;

declare_id!("G8BgM1hwZjPWv8jkJhwpj1WKVneuUUuK9QKXDJxJtX2u");

#[program]
pub mod locked_voter {
    use super::*;

    pub fn new_lock(ctx: Context<NewLock>, bump: u8) -> Result<()> {
        let lock_acc = &mut ctx.accounts.lock;

        let lock = Lock {
            base: ctx.accounts.base.key(),
            bump,
            token_mint: ctx.accounts.token_mint.key(),
        };

        lock_acc.set_inner(lock);

        Ok(())
    }

    pub fn new_escrow(ctx: Context<NewEscrow>) -> Result<()> {
        Ok(())
    }
}
