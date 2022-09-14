use crate::*;

#[event]
pub struct PollCreateEvent {
    /// The poll being created
    #[index]
    pub poll: Pubkey,
    /// The index of the [Proposal]
    pub index: u64,
}

#[event]
pub struct PollActivateEvent {
    /// The poll being created
    #[index]
    pub poll: Pubkey,
    pub voting_ends_at: i64,
}

#[event]
pub struct PollCancelEvent {
    /// The poll being created
    #[index]
    pub poll: Pubkey,
    pub voting_cancel_at: i64,
}
