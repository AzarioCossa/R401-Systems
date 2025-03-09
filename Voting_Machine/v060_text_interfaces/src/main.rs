use v060_text_interfaces::app_builder::run_app;
use v060_text_interfaces::configuration::Configuration;
use clap::Parser;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Configuration::parse();
    run_app(config).await?;
  
    Ok(())
}