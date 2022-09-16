use crate::*;
use mpl_token_metadata::state::Metadata;

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
    pub original_mint_metadata: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitEntry>, bump: u8) -> Result<()> {
    let mint_metadata_data = ctx.accounts.original_mint_metadata.try_borrow_mut_data()?;
    let original_mint_metadata = Metadata::deserialize(&mut mint_metadata_data.as_ref())?;
    if original_mint_metadata.mint != ctx.accounts.original_mint.key() {
        return Err(error!(errors::ErrorCode::InvalidMintMetadata));
    }

    let stake_entry = &mut ctx.accounts.stake_entry;

    let new_stake_entry = StakeEntry {
        bump,
        pool: ctx.accounts.stake_pool.key(),
        amount: 0,
        original_mint: ctx.accounts.original_mint.key(),
        stake_mint: None,
    };

    stake_entry.set_inner(new_stake_entry);

    Ok(())
}
