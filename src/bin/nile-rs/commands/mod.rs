mod compile;
mod counterfactual_address;
mod init;
mod run;
mod setup;

pub use compile::{
    cairo_to_sierra::CompileCairoToSierra, sierra_to_casm::CompileSierraToCasm, Compile,
};
pub use counterfactual_address::CounterfactualAddress;
pub use init::Init;
pub use run::Run;
pub use setup::Setup;

use anyhow::Result;
use async_trait::async_trait;

/// Common trait for Cli commands
#[async_trait]
pub trait CliCommand {
    type Output;
    async fn run(&self) -> Result<Self::Output>;
}
