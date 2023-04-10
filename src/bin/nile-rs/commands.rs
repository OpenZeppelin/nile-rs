mod call;
mod clean;
mod compile;
mod counterfactual_address;
mod declare;
mod deploy;
mod get_accounts;
mod get_balance;
mod get_nonce;
mod init;
mod legacy_deploy;
mod run;
mod send;
mod setup;
mod status;

pub use call::Call;
pub use clean::Clean;
pub use compile::Compile;
pub use counterfactual_address::CounterfactualAddress;
pub use declare::{DeclareV1, DeclareV2 as Declare};
pub use deploy::Deploy;
pub use get_accounts::GetAccounts;
pub use get_balance::GetBalance;
pub use get_nonce::GetNonce;
pub use init::Init;
pub use legacy_deploy::LegacyDeploy;
pub use run::Run;
pub use send::Send;
pub use setup::Setup;
pub use status::Status;

use anyhow::Result;
use async_trait::async_trait;

/// Common trait for Cli commands
#[async_trait]
pub trait CliCommand {
    type Output;
    async fn run(&self) -> Result<Self::Output>;
}
