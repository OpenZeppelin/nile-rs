mod cli;
mod commands;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        cli::Commands::Compile {} => commands::compile(),
    }
}