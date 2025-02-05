use tokio::io::{self, AsyncBufReadExt, BufReader};
use std::env;
use v020_app_builder::{run_app, configuration::Configuration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut candidates = Vec::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--candidates" || args[i] == "-c"{
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
    run_app(config).await
}