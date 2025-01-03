use anchor_lang::prelude::*;

pub mod state;

pub use state::*;

declare_id!("H5tnBxzDNQ5c3hZDpXbrJejATxchkLWknXfM1fA2JmyG");

#[program]
pub mod dao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let dao_state = &mut ctx.accounts.dao_state;
        dao_state.treasury_balance = 0;
        dao_state.total_proposals = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8+16,
    )]
    pub dao_state: Account<'info, DaoState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinDao<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8+8,
    )]
    pub member_account: Account<'info, MemberAccount>,
    pub dao_state: Account<'info, DaoState>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub dao_state: Account<'info, DaoState>,
    #[account(init, payer = user, space = 8+256)]
    pub proposal_account: Account<'info, ProposalAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub proposal_account: Account<'info, ProposalAccount>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub dao_state: Account<'info, DaoState>,
    #[account(mut)]
    pub proposal_account: Account<'info, ProposalAccount>,
}

pub enum CustomError {
    #[msg("Proposal has already been executed.")]
    ProposalAlreadyExecuted,
    #[msg("Proposal was rejected.")]
    ProposalRejected,
}
