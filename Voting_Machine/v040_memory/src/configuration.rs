use clap::Parser; 

#[derive(Debug, Parser)]
pub struct Configuration {
    #[clap(short = 'c', long = "candidates", value_name = "CANDIDATE", help = "Liste des candidats à l'élection")]
    pub candidates: Vec<String>,
}