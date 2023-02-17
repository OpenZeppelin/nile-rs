mod cli;
mod commands;
mod config;

use anyhow::{Ok, Result};

use clap::Parser;
use cli::Cli;
use commands::CliCommand;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Compile(cmd) => {
            cmd.run()?;
        }
        cli::Commands::CompileCairo(cmd) => {
            cmd.run()?;
        }
        cli::Commands::CompileSierra(cmd) => {
            cmd.run()?;
        }
        cli::Commands::Init(cmd) => {
            cmd.run()?;
        }
    }

    Ok(())
}