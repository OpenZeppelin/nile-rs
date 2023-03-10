mod call;
mod counterfactual_address;
mod declare;
mod deploy;
mod init;
mod run;
mod send;
mod setup;

pub use call::Call;
pub use counterfactual_address::CounterfactualAddress;
pub use declare::Declare;
pub use deploy::Deploy;
pub use init::Init;
pub use run::Run;
pub use send::Send;
pub use setup::Setup;

use anyhow::Result;
use async_trait::async_trait;

/// Common trait for Cli commands
#[async_trait]
pub trait CliCommand {
    type Output;
    async fn run(&self) -> Result<Self::Output>;
}
