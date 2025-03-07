use anyhow::Result;
use clap::Parser;
use tokio::io::{self, AsyncBufReadExt, BufReader};
mod configuration;
mod domain;
mod app_builder;
use configuration::Configuration;
use app_builder::run_app;


#[tokio::main]
async fn main() -> Result<()> {
    let config = Configuration::parse();
    run_app(config).await;
    Ok(())
}