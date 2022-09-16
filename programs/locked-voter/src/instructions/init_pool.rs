use crate::*;

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(
        init,
        seeds = [
            b"my_khe_stake_pool".as_ref()
        ],
        payer = payer,
        bump,
        space = 8
    )]
    pub stake_pool: Account<'info, StakePool>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitPool>, bump: u8) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;

    let new_stake_pool = StakePool {
        bump,
        authority: ctx.accounts.payer.key(),
        requires_creator: ctx.accounts.payer.key(),
        total_staked: 0,
    };

    stake_pool.set_inner(new_stake_pool);
    Ok(())
}
