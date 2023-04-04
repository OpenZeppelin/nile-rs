use crate::commands::{
    Call, Compile, CounterfactualAddress, Declare, DeclareV1, Deploy, GetAccounts, GetBalance,
    GetNonce, Init, Run, Send, Setup, Status,
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

    #[clap(about = "Build the project using Scarb")]
    Compile(Compile),

    #[clap(about = "Get counterfactual address from signer")]
    CounterfactualAddress(CounterfactualAddress),

    #[clap(about = "Declare a contract through an Account")]
    Declare(Declare),

    #[clap(about = "Declare a legacy contract through an Account (Cairo 0)")]
    LegacyDeclare(DeclareV1),

    #[clap(about = "Deploy a contract through an Account")]
    Deploy(Deploy),

    #[clap(about = "Execute a script from the scripts folder")]
    Run(Run),

    #[clap(about = "Execute a transaction through an Account")]
    Send(Send),

    #[clap(about = "Deploy and setup an Account contract (OZ version)")]
    Setup(Setup),

    #[clap(about = "Query the status of a transaction")]
    Status(Status),

    #[clap(about = "Query the nonce from an address")]
    GetNonce(GetNonce),

    #[clap(about = "Query the balance from an address")]
    GetBalance(GetBalance),

    #[clap(about = "Query the registered accounts from the given network")]
    GetAccounts(GetAccounts),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
