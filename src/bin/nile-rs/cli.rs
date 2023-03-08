use crate::commands::{
    Call, Compile, CompileCairoToSierra, CompileSierraToCasm, CounterfactualAddress, Declare,
    Deploy, Init, Run, Send, Setup,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nile")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(
        global = true,
        default_value = "localhost",
        long,
        short,
        help = "Default to localhost (port 5050)"
    )]
    network: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Initialize a new Nile project")]
    Init(Init),

    #[clap(about = "Query the blockchain with a function call")]
    RawCall(Call),

    #[clap(about = "Compile Cairo1 contracts")]
    Compile(Compile),

    #[clap(about = "Compile Cairo1 contracts to Sierra")]
    CompileCairo(CompileCairoToSierra),

    #[clap(about = "Compile Sierra artifacts to Casm")]
    CompileSierra(CompileSierraToCasm),

    #[clap(about = "Get counterfactual address from signer")]
    CounterfactualAddress(CounterfactualAddress),

    #[clap(about = "Declare a contract through an Account")]
    Declare(Declare),

    #[clap(about = "Deploy a contract through an Account")]
    Deploy(Deploy),

    #[clap(about = "Execute a script from the scripts folder")]
    Run(Run),

    #[clap(about = "Execute a transaction through an Account")]
    Send(Send),

    #[clap(about = "Deploy and setup an Account contract (OZ version)")]
    Setup(Setup),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
