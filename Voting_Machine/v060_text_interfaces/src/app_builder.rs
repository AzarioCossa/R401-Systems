use crate::configuration::Configuration;
use crate::configuration::StorageType;
use crate::domain::Candidate;
use crate::domain::VoteOutcome;
use crate::domain::VotingMachine;
use crate::storage::Storage;
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::use_cases::VoteForm;
use crate::use_cases::VotingController;
use crate::interfaces::cli_interface::{handle_lines};

use tokio::io::{self, AsyncBufReadExt, BufReader};
use crate::interfaces::Lexicon;

pub fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let mut candidates: Vec<Candidate> = vec![];

    for candidate in &configuration.candidates {
        candidates.push(Candidate(candidate.clone()));
    }

    VotingMachine::new(candidates)
}

pub async fn run_app(config: Configuration) -> anyhow::Result<()> 
{
    match config.storage_type {
        StorageType::File => {
            handle_lines::<FileStore>(config).await
        },
        StorageType::Memory => {
            handle_lines::<MemoryStore>(config).await
        }
    }
}
