use anchor_lang::prelude::*;

declare_id!("31jJwhAZtp87oh3qHX9oczb1o2nncRJqkQru1UDUzJT1");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
