use anyhow::Result;

use async_trait::async_trait;
use clap::Parser;
use nile_rs::common::getters::get_nonce;

use super::CliCommand;

#[derive(Parser, Debug)]
pub struct GetNonce {
    #[clap(help = "The contract address")]
    pub address: String,

    #[clap(from_global)]
    network: String,
}

#[async_trait]
impl CliCommand for GetNonce {
    type Output = ();

    // Get the nonce from the provided address
    async fn run(&self) -> Result<Self::Output> {
        let nonce = get_nonce(&self.address, &self.network).await?;

        println!("\nThe current nonce is: {:#}", nonce);

        Ok(())
    }
}
