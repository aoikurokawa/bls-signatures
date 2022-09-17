use anchor_spl::token::TokenAccount;

use {
    crate::*
};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        mut, 
        seeds = [b"my_khe_stake_entry".as_ref()], 
        bump = stake_entry.bump
    )]
    pub stake_entry: Account<'info, StakeEntry>,
    #[account(
        mut, 
        constraint = stake_entry.pool == stake_pool.key() 
    )]
    pub stake_pool: Account<'info, StakePool>,
    #[account(
        mut,
        constraint = 
            stake_entry_original_token_account.mint == stake_entry.original_mint && 
            stake_entry_original_token_account.owner == stake_entry.key()
    )]
    pub stake_entry_original_token_account: Account<'info, TokenAccount>,
    pub original_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,
}

pub fn handler(ctx: Context<Stake>) -> Result<()> {
    let stake_entry = &mut ctx.accounts.stake_entry;
    let stake_pool = &mut ctx.accounts.stake_pool;
    Ok(())
}