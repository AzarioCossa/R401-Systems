use std::collections::HashMap;
use std::env;
use crate::configuration::Configuration;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use anyhow::Result;

pub async fn run_app(_configuration: Configuration) -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut candidates = Vec::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--candidates" || args[i] == "-c" {
            i += 1;
            while i < args.len() && !args[i].starts_with("--") {
                candidates.push(args[i].clone());
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    let config = Configuration { candidates };

    let mut scores = HashMap::new();
    for candidate in &config.candidates {
        scores.insert(candidate.clone(), 0);
    }

    let normal_vote = 2;
    let abstention_vote = 1;
    let blank_vote = 0;

    

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
                        voteurs.insert(voter_name.to_string(), normal_vote);
                        println!("{} a voté pour {}", voter_name, candidate_name);
                    }
                } else {
                    println!("{} a voté nul", voter_name);
                    voteurs.insert(voter_name.to_string(), abstention_vote);
                }
            }

            ["voter", voter_name] => {
                println!("{} a voté blanc", voter_name);
                voteurs.insert(voter_name.to_string(), blank_vote);
                voters.push(voter_name.to_string());
            }

            _ => {
                println!("Commande invalide");
            }
        }
    }
    Ok(())
}