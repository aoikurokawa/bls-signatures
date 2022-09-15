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

    pub fn new_escrow(ctx: Context<NewEscrow>, bump: u8) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        let new_escrow = Escrow {
            lock: ctx.accounts.lock.key(),
            owner: ctx.accounts.escrow_owner.key(),
            bump,
            tokens: anchor_spl::associated_token::get_associated_token_address(
                &escrow.key(),
                &ctx.accounts.lock.token_mint,
            ),
            amount: 1,
            escrow_started_at: 0,
            escrow_ends_at: 0,
        };

        escrow.set_inner(new_escrow);

        Ok(())
    }
}
