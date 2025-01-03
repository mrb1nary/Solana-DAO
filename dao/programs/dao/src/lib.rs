use anchor_lang::prelude::*;

pub mod state;

pub use state::*;

declare_id!("H5tnBxzDNQ5c3hZDpXbrJejATxchkLWknXfM1fA2JmyG");

#[program]
pub mod dao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
