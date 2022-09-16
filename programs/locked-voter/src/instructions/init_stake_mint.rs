use {
    anchor_lang::solana_program::program::invoke,
    anchor_lang::solana_program::system_instruction::create_account,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{initialize_mint, InitializeMint, Token},
    },
    spl_token,
};

use anchor_lang::solana_program::{program::invoke_signed, program_pack::Pack};
use anchor_spl::associated_token;
use mpl_token_metadata::{
    instruction::create_metadata_accounts_v2,
    state::{Creator, Metadata},
};

use crate::*;

#[derive(Accounts)]
pub struct InitStakeMint<'info> {
    #[account(mut)]
    pub stake_pool: Account<'info, StakePool>,
    #[account(mut)]
    pub stake_entry: Account<'info, StakeEntry>,

    pub original_mint: Account<'info, Mint>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub original_mint_metadata: AccountInfo<'info>,

    #[account(mut)]
    pub vault_mint: Signer<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub vault_mint_metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub stake_entry_vault_mint_token_account: UncheckedAccount<'info>,

    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token: Program<'info, AssociatedToken>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(address = mpl_token_metadata::id())]
    pub token_metadata_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitStakeMint>, name: String, symbol: String) -> Result<()> {
    let stake_entry = &mut ctx.accounts.stake_entry;
    let stake_pool_seeds = &[b"my_khe_stake_pool".as_ref()];
    let stake_pool_signer = &[&stake_pool_seeds[..]];
    stake_entry.stake_mint = Some(ctx.accounts.vault_mint.key());

    // crate mint
    invoke(
        &create_account(
            ctx.accounts.payer.key,
            ctx.accounts.vault_mint.key,
            ctx.accounts
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN as u64,
            &spl_token::id(),
        ),
        &[
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.vault_mint.to_account_info(),
        ],
    )?;

    // Initialize mint
    let cpi_accounts = InitializeMint {
        mint: ctx.accounts.vault_mint.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    initialize_mint(
        cpi_context,
        0,
        &ctx.accounts.stake_pool.key(),
        Some(&ctx.accounts.stake_pool.key()),
    )?;

    // create associated token account for stake_entry
    let cpi_accounts = associated_token::Create {
        payer: ctx.accounts.payer.to_account_info(),
        associated_token: ctx.accounts.associated_token.to_account_info(),
        authority: stake_entry.to_account_info(),
        mint: ctx.accounts.vault_mint.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    associated_token::create(cpi_context)?;

    // create metadata
    let mut metadata_uri_param = "".to_string();
    if !ctx.accounts.original_mint_metadata.data_is_empty() {
        let mint_metadata_data = ctx.accounts.original_mint_metadata.try_borrow_data()?;
        let original_mint_metadata = Metadata::deserialize(&mut mint_metadata_data.as_ref())?;
        metadata_uri_param = original_mint_metadata
            .data
            .uri
            .trim_matches(char::from(0))
            .to_string();
    }

    invoke_signed(
        &create_metadata_accounts_v2(
            *ctx.accounts.token_metadata_program.key,
            *ctx.accounts.vault_mint_metadata.key,
            *ctx.accounts.vault_mint.key,
            ctx.accounts.stake_pool.key(),
            *ctx.accounts.payer.key,
            ctx.accounts.stake_pool.key(),
            name,
            symbol,
            metadata_uri_param,
            Some(vec![
                Creator {
                    address: ctx.accounts.stake_pool.key(),
                    verified: true,
                    share: 50,
                },
                Creator {
                    address: ctx.accounts.stake_entry.key(),
                    verified: true,
                    share: 50,
                },
            ]),
            1,
            true,
            true,
            None,
            None,
        ),
        &[
            ctx.accounts.vault_mint_metadata.to_account_info(),
            ctx.accounts.vault_mint.to_account_info(),
            ctx.accounts.stake_pool.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),

        ],
        stake_pool_signer,
    )?;

    Ok(())
}
