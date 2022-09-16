use crate::*;

#[derive(Accounts)]
pub struct InitEntry<'info> {
    /// Staking pool
    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,

    #[account(
        init,
        seeds = [
            b"my_khe_stake_entry".as_ref()
        ],
        bump,
        payer = payer,
        space = 8
    )]
    pub stake_entry: Account<'info, StakeEntry>,

    pub original_mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    original_mint_metadata: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitEntry>, bump: u8) -> Result<()> {
    let stake_entry = &mut ctx.accounts.stake_entry;

    let new_stake_entry = StakeEntry {
        bump,
        pool: ctx.accounts.stake_pool.key(),
        amount: 0,
        original_mint: ctx.accounts.original_mint.key(),
    };

    stake_entry.set_inner(new_stake_entry);

    Ok(())
}
