use async_trait::async_trait;

use crate::{domain::VotingMachine, storage::Storage};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MemoryStore {
    voting_machine: VotingMachine, 
}

#[async_trait]
impl Storage for MemoryStore {
    async fn new(machine: VotingMachine) -> anyhow::Result<Self> {
        Ok(Self {
            voting_machine: machine,
        })
    }

    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        Ok(self.voting_machine.clone())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
        self.voting_machine = machine;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Candidate;

    #[tokio::test]
async fn test_get_equals_put() {
    use super::*;
    let candidates = vec![Candidate("Alice".to_string()), Candidate("Bob".to_string())];
    let voting_machine = VotingMachine::new(candidates.clone());

    let mut memory_store = MemoryStore::new(VotingMachine::new(candidates.clone())).await.expect("Falha ao criar MemoryStore");

    memory_store.put_voting_machine(VotingMachine::new(candidates)).await.expect("Falha ao inserir VotingMachine");

    let retrieved_vm = memory_store.get_voting_machine().await.expect("Falha ao obter VotingMachine");

    assert_eq!(voting_machine, retrieved_vm);
}

}