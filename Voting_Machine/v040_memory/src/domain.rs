use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone,Debug)]
pub struct Voter(pub String);

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone,Debug)]
pub struct Candidate(pub String);

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
pub struct Score(pub usize);

#[derive(Ord, PartialOrd, PartialEq, Eq,Debug, Clone)]
pub struct AttendenceSheet(pub Set<Voter>);

#[derive(Ord, PartialOrd, PartialEq, Eq,Debug, Clone)]
pub struct Scoreboard {
  pub scores: Map<Candidate, Score>,
  pub blank_score: Score,
  pub invalid_score: Score,
}

impl Scoreboard {
  pub fn new(candidates: Vec<Candidate>) -> Self { 
    let mut scores : Map<Candidate, Score> = Map::new();
  
   for candidate in candidates {
      scores.insert(candidate, Score(0));
   }

   Self {
     scores,
     blank_score: Score(0),
     invalid_score: Score(0),
   }
  }
}


pub struct BallotPaper {
  pub voter: Voter,
  pub candidate: Option<Candidate>,
}


pub enum VoteOutcome {
  AcceptedVote(Voter, Candidate),
  BlankVote(Voter),
  InvalidVote(Voter),
  HasAlreadyVoted(Voter),
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct VotingMachine {
  voters: AttendenceSheet,
  scoreboard: Scoreboard,
}

impl VotingMachine {
  pub fn new(candidates: Vec<Candidate>) -> VotingMachine { 
    Self {
      voters: AttendenceSheet(Set::new()),
      scoreboard: Scoreboard::new(candidates),
    }
  }

  pub fn vote(&mut self, ballotPaper: BallotPaper) -> VoteOutcome {
    if self.voters.0.contains(&ballotPaper.voter) {
      return VoteOutcome::HasAlreadyVoted(ballotPaper.voter);
    }

    match ballotPaper.candidate {
      Some(candidate) => {
        self.scoreboard.scores.get_mut(&candidate).unwrap().0 += 1;
        match self.scoreboard.scores.get(&candidate) {
          Some(_) => { 
            self.voters.0.insert(ballotPaper.voter.clone());
            VoteOutcome::AcceptedVote(ballotPaper.voter, candidate)
          },
          None => {
            self.voters.0.insert(ballotPaper.voter.clone());
            VoteOutcome::InvalidVote(ballotPaper.voter)
          },
        }
      }
      None => {
        self.scoreboard.blank_score.0 += 1;
        self.voters.0.insert(ballotPaper.voter.clone());
        VoteOutcome::BlankVote(ballotPaper.voter)
      }
    }
  }

  pub fn get_scoreboard(&self) -> &Scoreboard {
    &self.scoreboard
  }

  pub fn get_voters(&self) -> &AttendenceSheet {
    &self.voters
  }
}



#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
  assert_eq!(1 + 1, 2);
  }
}
