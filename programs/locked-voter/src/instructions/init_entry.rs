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
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>
}