use anyhow::{Ok, Result};
use async_trait::async_trait;
use clap::Parser;
use nile_rs::core::status::get_tx_status;

use super::CliCommand;

#[derive(Parser, Debug)]
pub struct Status {
    #[clap(help = "The transaction hash")]
    pub tx_hash: String,

    #[clap(
        short,
        long,
        help = "Block until the transaction gets either ACCEPTED or REJECTED",
        default_value_t = false
    )]
    pub track: bool,

    #[clap(from_global)]
    network: String,
}

#[async_trait]
impl CliCommand for Status {
    type Output = ();

    // Query the status of a transaction
    async fn run(&self) -> Result<Self::Output> {
        get_tx_status(&self.tx_hash, &self.network, self.track).await?;
        Ok(())
    }
}
