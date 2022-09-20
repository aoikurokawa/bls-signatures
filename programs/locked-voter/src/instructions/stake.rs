use anchor_spl::token::{TokenAccount, Token, self};

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
    #[account(
        mut,
        constraint = payer_original_mint_token_account.amount > 0 &&
            payer_original_mint_token_account.mint == stake_entry.original_mint && 
            payer_original_mint_token_account.owner == payer.key(),
    )]
    pub payer_original_mint_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let stake_entry = &mut ctx.accounts.stake_entry;
    let stake_pool = &mut ctx.accounts.stake_pool;

    // transfer original mint
    let cpi_accounts = Transfer {
        from: ctx.accounts.payer_original_mint_token_account.to_account_info(),
        to: ctx.accounts.stake_entry_original_token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_context, amount)?;

    // update stake_entry and stake_pool
    stake_entry.amount = stake_entry.amount.checked_add(amount).unwrap();
    stake_pool.total_staked = stake_pool.total_staked.checked_add(1).expect("Add error");
    Ok(())
}