use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Proposal has already been executed.")]
    ProposalAlreadyExecuted,
    #[msg("Proposal was rejected.")]
    ProposalRejected,
}
