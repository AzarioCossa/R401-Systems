use clap::Parser;
use clap::ValueEnum;

#[derive(Clone,Copy, ValueEnum, Debug)]
pub enum StorageType {
    File,
    Memory,
}

#[derive(Clone,Copy, ValueEnum, Debug)]
pub enum Language {
    FR,
    EN,
}

#[derive(Debug, Parser)]
pub struct Configuration{
    #[arg(short = 'c',long = 'candidates', required = true, num_args = 1..)]
    pub candidates :  Vec<String>,

    #[arg(short = 's',long = 'storage', required = true, num_args = 1)]
    pub storage_type :  StorageType,

    #[arg(short = 'l',long = 'language', required = true, num_args = 1)]
    pub language :  Language,
}
