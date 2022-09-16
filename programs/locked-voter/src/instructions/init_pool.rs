use crate::*;

#[derive(Accounts)]
pub struct InitPool<'info> {
    #[account(
        init,
        seeds = [
            b"my_khe_stake_pool"
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
