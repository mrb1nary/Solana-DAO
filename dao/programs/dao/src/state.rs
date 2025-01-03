use anchor_lang::prelude::*;

#[account]
pub struct DaoState {
    pub treasury_balance: u64,
    pub total_proposals: u64,
}

#[account]
pub struct MemberAccount {
    pub is_member: bool,
}

#[account]
pub struct ProposalAccount {
    pub title: String,
    pub description: String,
    pub upvotes: u64,
    pub downvotes: u64,
    pub is_executed: bool,
    pub amount_requested: u64,
}
