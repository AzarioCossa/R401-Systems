use tokio::io::{self, AsyncBufReadExt, BufReader};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let candidates = ["Alfa", "Beta", "Charlie"];
    let mut scores = HashMap::new();
    for &candidate in &candidates {
        scores.insert(candidate, 0);
    }

    let normalVote = 2;
    let abstentionVote = 1;
    let blankVote = 0;

    let mut voters = Vec::<String>::new();
    let mut voteurs: HashMap<String, u8> = HashMap::new();

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts.as_slice() {
            ["votants"] => {
                println!("Liste des votants:");
                for voter in &voters {
                    println!("{}", voter);
                }
            }
            ["score"] => {
                println!("Candidats et leur score:");
                for (candidate, score) in &scores {
                    println!("{}: {}", candidate, score);
                }
            }
            ["voter", voter_name, candidate_name] => {
                if scores.contains_key(*candidate_name) {
                    if voters.contains(&voter_name.to_string()) {
                        println!("{} a déjà voté", voter_name);
                    } else {
                        *scores.get_mut(*candidate_name).unwrap() += 1;
                        voters.push(voter_name.to_string());
                        voteurs.insert(voter_name.to_string(), normalVote);
                        println!("{} a voté pour {}", voter_name, candidate_name);
                    }
                } else {
                    println!("{} a voté nul", voter_name);
                    voteurs.insert(voter_name.to_string(), abstentionVote);
                }
            }

            ["voter", voter_name] => {
                println!("{} a voté blanc", voter_name);
                voteurs.insert(voter_name.to_string(), blankVote);
                voters.push(voter_name.to_string());
            }

            _ => {
                println!("Commande invalide");
            }
        }
    }

    Ok(())
}