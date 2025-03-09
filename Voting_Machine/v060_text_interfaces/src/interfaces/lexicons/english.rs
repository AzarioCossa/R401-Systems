use crate::interfaces::lexicon::Lexicon;

impl Lexicon{
    pub fn english()-> Self{
        Self{
            blank: "Blank",
            candidate: "Candidate",
            voter: "Voter",
            has_voted_null: "has voted null",
            has_voted_blank: "has voted blank",
            has_already_voted: "has already voted",
            has_voted_for: "has voted for",
            current_score: "Current scores",
            menu: r#"
                There are 4 available commands:
                1) vote Tux Nixos -> Vote for Nixos as Tux
                2) vote Tux -> Vote blank as Tux
                3) voters -> Display the list of voters
                4) scores -> Display the scores of the candidates
                "#,
            invalid_command_vote: "Invalid 'vote' command, please specify a voter.",
            unknown_command: "Unknown command. Type a valid command.",
            scores: "Scores",
        }
    }
}