mod cli;
mod commands;
mod common;
mod utils;

use anyhow::{Ok, Result};

use clap::Parser;
use cli::Cli;
use commands::compile::Compiler;

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
    }

    Ok(())
}
