use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct DaoState {
    pub treasury_balance: u64,
    pub total_proposals: u64,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct MemberAccount {
    pub is_member: bool,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct ProposalAccount {
    #[max_len(64)]
    pub title: String,
    #[max_len(300)]
    pub description: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub is_executed: bool,
    pub amount_requested: u64,
}
