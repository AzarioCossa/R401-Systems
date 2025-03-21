use clap::Parser;
use clap::ValueEnum;

#[derive(Clone,Copy, ValueEnum, Debug)]
pub enum StorageType {
    File,
    Memory,
}

#[derive(Debug, Parser)]
pub struct Configuration{
    #[arg(short,long, required = true, num_args = 1..)]
    pub candidates :  Vec<String>,

    #[arg(short,long, required = true, num_args = 1)]
    pub storage_type :  StorageType,
}
