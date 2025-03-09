use clap::Parser; 
use clap::ValueEnum;

#[derive(Debug, Parser)]
pub struct Configuration {
    #[clap(short = 'c', long = "candidates", value_name = "CANDIDATE", help = "Liste des candidats à l'élection")]
    pub candidates: Vec<String>,

    #[clap(short = 's', long = "storage", value_enum, default_value = "memory", help = "Type de stockage à utiliser")]
    pub storage: StorageType,
}


#[derive(Clone, Copy, ValueEnum)]
pub enum StorageType {
  File,
  Memory,
}
