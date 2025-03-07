use std::collections::HashMap;
use std::env;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use anyhow::Result;
use crate::{
    configuration::Configuration, domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine}, storage::Storage, storages::memory::MemoryStore
};

fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let mut candidates = Vec::new();
    for candidate in &configuration.candidates {
        candidates.push(Candidate(candidate.clone()));
    }

    VotingMachine::new(candidates)
}


pub async fn run_app(_configuration: Configuration) -> Result<()> {
    let mut voting_machine = create_voting_machine(&_configuration);
    let mut memory_store = MemoryStore::new(voting_machine).await?;
    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(line) = lines.next_line().await? {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("votants") => {
                println!("Liste des votants:");
                println!("{:?}", voting_machine.get_voters());
                
            }
            Some("score") => {
                println!("Candidats et leur score:");
                println!("{:?}", voting_machine.get_scoreboard());
            }
            Some("voter") => match parts.next() {
                Some(voter) => {
                    let voter = Voter(voter.to_string());
                    let candidate = match parts.next() {
                        Some(candidate) => Some(Candidate(candidate.to_string())),
                        None => None,
                    };

                    let ballot_paper = BallotPaper { voter: voter.clone(), candidate };
                    let result = voting_machine.vote(ballot_paper);
                    memory_store.put_voting_machine(voting_machine).await?;

                    match result {
                        VoteOutcome::AcceptedVote(_, _) => println!("Vote accepté"),
                        VoteOutcome::BlankVote(_) => println!("Vote blanc"),
                        VoteOutcome::InvalidVote(_) => println!("Vote invalide"),
                        VoteOutcome::HasAlreadyVoted(_) => println!("Le votant a déjà voté"),
                    }
                    
                }
                None => {
                    println!("Problème de syntaxe avec la commande voter");
                }
            }
            Some(_) => {
                println!("Commande inconnue");
            }
            None => {
                println!("Commande invalide");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::configuration::Configuration;

    #[tokio::test]
    async fn test_voting_logic() {
        use crate::app_builder::run_app;
        use crate::configuration::Configuration;
        use std::process::{Command, Stdio};
        use std::io::Write;

        let mut child = Command::new("cargo")
            .args(["run", "--", "--candidates", "Alice", "Bob"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start process");

        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        writeln!(stdin, "voter John Alice").unwrap();
        writeln!(stdin, "voter Jane Bob").unwrap();
        writeln!(stdin, "score").unwrap();

        let output = child.wait_with_output().expect("Failed to read stdout");
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("John a voté pour Alice"));
        assert!(stdout.contains("Jane a voté pour Bob"));
        assert!(stdout.contains("Alice: 1"));
        assert!(stdout.contains("Bob: 1"));
    }
}