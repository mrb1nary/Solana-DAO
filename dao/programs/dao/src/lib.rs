use anchor_lang::prelude::*;

pub mod error;
pub mod state;

pub use error::*;
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

    pub fn join_dao(ctx: Context<JoinDao>, membership_fee: u64) -> Result<()> {
        let dao_state = &mut ctx.accounts.dao_state;
        dao_state.treasury_balance += membership_fee;

        let member_account = &mut ctx.accounts.member_account;
        member_account.is_member = true;
        Ok(())
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title: String,
        description: String,
    ) -> Result<()> {
        let dao_state = &mut ctx.accounts.dao_state;
        dao_state.total_proposals += 1;

        let proposal_account = &mut ctx.accounts.proposal_account;
        proposal_account.title = title;
        proposal_account.description = description;
        proposal_account.upvotes = 0;
        proposal_account.downvotes = 0;
        proposal_account.is_executed = false;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, approve: bool) -> Result<()> {
        let proposal_account = &mut ctx.accounts.proposal_account;

        if approve {
            proposal_account.upvotes += 1;
        } else {
            proposal_account.downvotes += 1;
        }
        Ok(())
    }

    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        let proposal_account = &mut ctx.accounts.proposal_account;
        let dao_state = &mut ctx.accounts.dao_state;

        require!(
            !proposal_account.is_executed,
            CustomError::ProposalAlreadyExecuted
        );
        require!(
            proposal_account.upvotes > proposal_account.downvotes,
            CustomError::ProposalRejected
        );

        dao_state.treasury_balance -= proposal_account.amount_requested;
        proposal_account.is_executed = true;
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
