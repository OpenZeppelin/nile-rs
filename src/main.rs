mod cli;
mod commands;

use anyhow::{Ok, Result};

use clap::Parser;
use cli::Cli;
use commands::compile::Compiler;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Compile {} => {
            commands::compile::compile()?;
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
