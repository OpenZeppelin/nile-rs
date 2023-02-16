use crate::commands::compile::{
    cairo_to_sierra::CompileCairoToSierra, sierra_to_casm::CompileSierraToCasm, Compile
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nile")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Compile Cairo1 contracts.")]
    Compile(Compile),

    #[clap(about = "Compile Cairo1 contracts to Sierra.")]
    CompileCairo(CompileCairoToSierra),

    #[clap(about = "Compile Sierra artifacts to Casm.")]
    CompileSierra(CompileSierraToCasm),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
