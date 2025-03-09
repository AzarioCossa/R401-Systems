use tokio::io::{self, AsyncBufReadExt, BufReader};
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
use crate::app_builder::{create_voting_machine};
use crate::domain::{Scoreboard, AttendenceSheet};


pub async fn handle_lines<Store: Storage>(config: Configuration, &lexicon : &Lexicon) -> anyhow::Result<()> {

    let voting_machine: VotingMachine = create_voting_machine(&config);
    let store = Store::new(voting_machine).await?;
    let mut voting_controller  = VotingController::new(store);

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(line) = lines.next_line().await? {
        let mut words = line.trim().split_whitespace();
        let voting_machine: VotingMachine = voting_controller.get_voting_machine().await?;

        match words.next() {
            Some(command) => match command {
                "voter" => match words.next() {
                    Some(voter) => {
                        let candidate: String = match words.next() {
                            Some(word) => word.to_string(),
                            None => String::from(""),
                        };
                        let ballot_paper: VoteForm = VoteForm {
                            voter: voter.to_string().clone(),
                            candidate: candidate,
                        };

                        let vote: VoteOutcome = voting_controller.vote(ballot_paper).await?;

                        match vote {
                            VoteOutcome::InvalidVote(voter) => {
                                println!("{:?} {:?}", voter,  lexicon.has_voted_null)
                            }
                            VoteOutcome::BlankVote(voter) => {
                                println!("{:?} {:?}", voter,  lexicon.has_voted_blank)
                            }
                            VoteOutcome::HasAlreadyVoted(voter) => {
                                println!("{:?} {:?}", voter,  lexicon.has_already_voted)
                            }
                            VoteOutcome::AcceptedVote(voter, candidat) => {
                                println!("{:?} {:?} {:?} {:?}", lexicon.voter, voter, lexicon.has_voted_for, candidat)
                            }
                        }
                    }
                    None => println!("{:?}", lexicon.invalid_command_vote),
                },

                "scores" => println!("{:?}", show_scoreboard(voting_machine.get_scoreboard())),
                "votants" => println!("Liste des votants : {:?}", voting_machine.get_voters()),
                _ => println!("{:?}", lexicon.unknown_command),
            },
            None => display_menu(),
        }
    }
    Ok(())
}

fn show_vote_outcome(outcome: VoteOutcome, lexicon : &Lexicon) -> String{
    match outcome {
        VoteOutcome::InvalidVote(voter) => format!("{:?} {:?}", voter,  lexicon.has_voted_null),
        VoteOutcome::BlankVote(voter) => format!("{:?} {:?}", voter,  lexicon.has_voted_blank),
        VoteOutcome::HasAlreadyVoted(voter) => format!("{:?} {:?}", voter,  lexicon.has_already_voted),
        VoteOutcome::AcceptedVote(voter, candidat) => format!("{:?} {:?} {:?} {:?}", lexicon.voter, voter, lexicon.has_voted_for,candidat),
    }
}
fn show_scoreboard(scoreboard: &Scoreboard, &lexicon : &Lexicon) -> String{
    let mut result = String::new();
    result.push_str(&format!("{:?}\n", lexicon.current_score));
    result.push_str(&format!("{:?}", scoreboard));
    result
}
fn show_attendence_sheet(attendence_sheet: &AttendenceSheet, &lexicon : &Lexicon) -> String{
    let mut result = String::new();
    result.push_str(&format!("{:?}\n", lexicon.current_score));
    result.push_str(&format!("{:?}", attendence_sheet));
    result
}

pub fn display_menu(lexicon: &Lexicon) {
    println!("{}", lexicon.menu);    
}