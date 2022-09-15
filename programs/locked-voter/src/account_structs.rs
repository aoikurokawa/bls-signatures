use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct NewLock<'info> {
    #[account(
        init,
        seeds = [
            b"my_khe_lock"
        ],
        bump,
        payer = payer,
        space = 8
    )]
    pub lock: Account<'info, Lock>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
