use anchor_lang::prelude::*;

use state::*;
mod state;
use constants::*;
mod constants;
use instructions::*;
mod instructions;

declare_id!("31jJwhAZtp87oh3qHX9oczb1o2nncRJqkQru1UDUzJT1");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializedConfig>) -> Result<()> {
        process_initialize_config(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, min_health_factor: u64) -> Result<()> {
        process_update_config(ctx, min_health_factor)
    }
}
