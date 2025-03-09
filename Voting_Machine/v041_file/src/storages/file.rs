use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::Path;
use crate::domain::VotingMachine;
use anyhow::Result;
use async_trait::async_trait;
use crate::{domain::VotingMachine, storage::Storage};


#[derive(Serialize, Deserialize)]
struct ScoreboardDao {
  scores: Map<String, usize>,
  blank_score: usize,
  invalid_score: usize,
}
#[derive(Serialize, Deserialize)]
pub struct VotingMachineDao {
  voters: Set<String>,
  scoreboard: ScoreboardDao,
}


struct FileStore{
    filepath: String,
}

const FILEPATH : &str = "machine.json";  

async fn store_voting_machine(machine: VotingMachine, filepath: &str) -> anyhow::Result<()>{
    let serialized = serde_json::to_string_pretty(machine)?;
    fs::write(filepath, serialized)?;
    Ok(())
}

impl FileStore {
    pub async fn create(machine: VotingMachine, filepath: &str) -> Result<Self> {
        if !Path::new(filepath).exists() {
            store_voting_machine(&machine, filepath).await?;
        }
        Ok(Self { filepath: filepath.to_string() })
    }
}

impl From<Scoreboard> for ScoreboardDao {
    fn from(scoreboard: Scoreboard) -> Self { 
        ScoreboardDao {
            scores: scoreboard.0,
            blank_score: scoreboard.1,
            invalid_score: scoreboard.2,
        }
    }
}
  
impl From<ScoreboardDao> for Scoreboard {
    fn from(scoreboard: ScoreboardDao) -> Self { 
        Scoreboard(scoreboard.scores, scoreboard.blank_score, scoreboard.invalid_score)
    }
}

impl From<VotingMachineDao> for VotingMachine {
    fn from(voting_machine: VotingMachineDao) -> Self { 
        VotingMachine(voting_machine.voters, voting_machine.scoreboard.into())
    }
}

impl From<VotingMachine> for VotingMachineDao {
    fn from(voting_machine: VotingMachine) -> Self { 
        VotingMachineDao {
            voters: voting_machine.0,
            scoreboard: voting_machine.1.into(),
        }
    }
}

#[async_trait]
impl Storage for FileStore {
    async fn new(machine: VotingMachine) -> Result<Self> {
        Self::create(machine, FILEPATH).await
    }

    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        let content = fs::read_to_string(&self.filepath)?;
        let machine: VotingMachine = serde_json::from_str(&content)?;
        Ok(machine)
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        store_voting_machine(machine, &self.filepath).await
    }
}
