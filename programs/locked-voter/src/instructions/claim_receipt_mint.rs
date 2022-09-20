use anchor_spl::{
    associated_token::AssociatedToken,
    token::{TokenAccount, Token, self}
};
use crate::*;

#[derive(Accounts)]
pub struct ClaimReceiptMint<'info> {
    #[account(mut)]
    pub stake_entry: Account<'info, StakeEntry>,

    pub original_mint: Account<'info, Mint>,
    #[account(mut)]
    pub receipt_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = stake_entry_receipt_mint_token_account.amount > 0
            && stake_entry_receipt_mint_token_account.owner == stake_entry.key() 
    )]
    pub stake_entry_receipt_mint_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = receipt_mint,
        associated_token::authority = payer
    )]
    pub user_receipt_mint_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>
}

pub fn handler(ctx: Context<ClaimReceiptMint>, amount: u64) -> Result<()> {
    let stake_entry = &mut ctx.accounts.stake_entry;
    // let user_pubkey = ctx.accounts.payer.key();
    
    let stake_entry_seed = [STAKE_ENTRY_PREFIX.as_bytes()];
    let stake_entry_signer = &[&stake_entry_seed[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.stake_entry_receipt_mint_token_account.to_account_info(),
        to: ctx.accounts.user_receipt_mint_token_account.to_account_info(),
        authority: stake_entry.to_account_info()
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(stake_entry_signer);
    token::transfer(cpi_context, amount)?;

    Ok(())

}