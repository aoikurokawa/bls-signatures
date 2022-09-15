use anchor_lang::prelude::*;

declare_id!("G8BgM1hwZjPWv8jkJhwpj1WKVneuUUuK9QKXDJxJtX2u");

#[program]
pub mod locked_voter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
