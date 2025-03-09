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


#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{configuration::Language, domain::{Candidate, VotingMachine}, storages::memory::MemoryStore};
    use std::collections::BTreeMap as Map;
    use std::collections::BTreeSet as Set;
    use super::*;

    #[tokio::test]
    async fn test_display_menu_if_no_command()
    {

        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Azário")));
        let voting_machine = VotingMachine::new(candidates);

        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
         
        
        let mut controller  = VotingController::new(store);
    
    
        assert_eq!(r#"
                Il y a 4 commandes disponibles :
                1) voter Tux Nixos -> Voter pour Nixos en tant que Tux
                2) voter Tux -> Voter blanc en tant que Tux
                3) votants -> Afficher la liste des votants
                4) scores -> Afficher les scores des candidats
                "#,handle_line("", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));
    }

    #[tokio::test]
    async fn test_display_voters(){
        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Azário")));
        let voting_machine = VotingMachine::new(candidates);
        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
  
        let lexicon: Lexicon = Lexicon::french();
        let mut controller  = VotingController::new(store);
        let line = "votants";
        assert_eq!("\"Scores actuels\"\nAttendenceSheet({})",handle_line(line, &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));
    }

    #[tokio::test]
    async fn test_display_scores(){
        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Azário")));
        let voting_machine = VotingMachine::new(candidates);
        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
        let mut controller  = VotingController::new(store);

        let result = "\"Scores actuels\"\nScoreboard { scores: {Candidate(\"Azário\"): Score(0)}, blank_score: Score(0), invalid_score: Score(0) }";

        assert_eq!(result,handle_line("scores", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));

    }

    #[tokio::test]
    async fn test_can_vote(){
        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Azário")));
        let voting_machine = VotingMachine::new(candidates);
        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
        let mut controller  = VotingController::new(store);

        let mut result = "\"Scores actuels\"\nScoreboard { scores: {Candidate(\"Azário\"): Score(0)}, blank_score: Score(0), invalid_score: Score(0) }";

        assert_eq!(result,handle_line("scores", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));

        let line = "voter Tux Azário";
        result = "\"Votant\" Voter(\"Tux\") \"a voté pour\" Candidate(\"Azário\")";
        assert_eq!(result,handle_line(line, &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));

        let line = "scores";
        result = "\"Scores actuels\"\nScoreboard { scores: {Candidate(\"Azário\"): Score(1)}, blank_score: Score(0), invalid_score: Score(0) }";
        assert_eq!(result,handle_line(line, &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));

    }

    async fn test_can_vote_blank(){
        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Azário")));
        let voting_machine = VotingMachine::new(candidates);
        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
        let mut controller  = VotingController::new(store);

        let mut result = "\"Scores actuels\"\nScoreboard { scores: {Candidate(\"Azário\"): Score(0)}, blank_score: Score(0), invalid_score: Score(0) }";

        assert_eq!(result,handle_line("scores", &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));

        let line = "voter Tux";
        result = "\"Votant\" Voter(\"Tux\") \"a voté pour\" \"blanc\"";
        assert_eq!(result,handle_line(line, &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));

        let line = "scores";
        result = "\"Scores actuels\"\nScoreboard { scores: {Candidate(\"Azário\"): Score(0)}, blank_score: Score(1), invalid_score: Score(0) }";
        assert_eq!(result,handle_line(line, &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));

    }

    async fn test_can_vote_when_no_voter(){
        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Azário")));
        let voting_machine = VotingMachine::new(candidates);
        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
        let mut controller  = VotingController::new(store);

        let line = "voter";
        let result = "Commande 'voter' invalide, veuillez spécifier un électeur.";
        assert_eq!(result,handle_line(line, &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));

    }

    async fn test_can_unknown_command(){
        let mut  candidates = vec![];
        candidates.push(Candidate(String::from("Azário")));
        let voting_machine = VotingMachine::new(candidates);
        let store = MemoryStore::new(voting_machine).await.expect("erreur lors de la creation de la memoire");
        let lexicon: Lexicon = Lexicon::french();
        let mut controller  = VotingController::new(store);

        let line = "test_command";
        let result = "Commande inconnue. Tapez une commande valide.";
        assert_eq!(result,handle_line(line, &mut controller, &lexicon).await.expect("erreur lors de lecture de la ligne"));

    }
}