mod cli;
mod commands;

use anyhow::{Ok, Result};
use dotenv::dotenv;

use clap::Parser;

use cli::Cli;
use commands::CliCommand;

fn main() -> Result<()> {
    // Load the environment variables from the ".env" file
    dotenv().ok();

    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Init(cmd) => {
            cmd.run()?;
        }
        cli::Commands::Compile(cmd) => {
            cmd.run()?;
        }
        cli::Commands::CompileCairo(cmd) => {
            cmd.run()?;
        }
        cli::Commands::CompileSierra(cmd) => {
            cmd.run()?;
        }
        cli::Commands::Run(cmd) => {
            cmd.run()?;
        }
        cli::Commands::CounterfactualAddress(cmd) => {
            cmd.run()?;
        }
    };

    Ok(())
}
