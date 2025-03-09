use crate::domain::VoteOutcome;
use crate::domain::VotingMachine;
use crate::storage::Storage;
use crate::use_cases::VoteForm;
use crate::use_cases::VotingController;
use crate::domain::{Scoreboard, AttendenceSheet};
use crate::interfaces::lexicon::Lexicon;


pub async fn handle_line<Store: Storage>(line: &str, voting_controller: &mut VotingController<Store>, lexicon : &Lexicon) -> anyhow::Result<String> {

        let mut words = line.trim().split_whitespace();
        let voting_machine: VotingMachine = voting_controller.get_voting_machine().await?;

        let result = match words.next() {
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
                        Ok(show_vote_outcome(vote, lexicon))
                    }
                    None => Ok(lexicon.invalid_command_vote.to_string()),
                },

                "scores" => Ok(show_scoreboard(voting_machine.get_scoreboard(), lexicon)),
                "votants" => Ok(show_attendence_sheet(voting_machine.get_voters(), lexicon)),
                _ =>Ok(lexicon.unknown_command.to_string()),
            },
            None => Ok(display_menu(lexicon)),
        };
        result
}

fn show_vote_outcome(outcome: VoteOutcome, lexicon : &Lexicon) -> String{
    match outcome {
        VoteOutcome::InvalidVote(voter) => format!("{:?} {:?}", voter,  lexicon.has_voted_null),
        VoteOutcome::BlankVote(voter) => format!("{:?} {:?}", voter,  lexicon.has_voted_blank),
        VoteOutcome::HasAlreadyVoted(voter) => format!("{:?} {:?}", voter,  lexicon.has_already_voted),
        VoteOutcome::AcceptedVote(voter, candidat) => format!("{:?} {:?} {:?} {:?}", lexicon.voter, voter, lexicon.has_voted_for,candidat),
    }
}

fn show_scoreboard(scoreboard: &Scoreboard, lexicon : &Lexicon) -> String{
    let mut result = String::new();
    result.push_str(&format!("{:?}\n", lexicon.current_score));
    result.push_str(&format!("{:?}", scoreboard));
    result.to_string()
}
fn show_attendence_sheet(attendence_sheet: &AttendenceSheet, lexicon : &Lexicon) -> String{
    let mut result = String::new();
    result.push_str(&format!("{:?}\n", lexicon.current_score));
    result.push_str(&format!("{:?}", attendence_sheet));
    result.to_string()
}

pub fn display_menu(lexicon: &Lexicon)-> String {
    lexicon.menu.to_string()   
}