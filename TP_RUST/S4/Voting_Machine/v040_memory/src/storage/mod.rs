pub mod memory;

use async_trait::async_trait;

use crate::domain::VotingMachine;

#[async_trait]
pub trait Storage{
    async get_voting_machine(&self) -> anyhow::Result<VotingMachine>;
    async put_voting_machine(&self, machine : VotingMachine) -> anyhow::Result<()>;
}