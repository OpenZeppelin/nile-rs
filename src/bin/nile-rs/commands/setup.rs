use anyhow::{Ok, Result};
use async_trait::async_trait;
use clap::Parser;

use super::CliCommand;
use nile_rs::core::accounts::account_factory::OZAccountFactory;

#[derive(Parser, Debug)]
pub struct Setup {
    #[clap(
        help = "Environment variable set to the private key",
        value_name = "PRIVATE_KEY_ENV"
    )]
    pub private_key_env: String,

    #[clap(help = "Salt for the address generation", long, short)]
    pub salt: Option<u32>,

    #[clap(help = "Max fee allowed to pay for the transaction", long, short)]
    pub max_fee: Option<u128>,

    #[clap(from_global)]
    network: String,
}

#[async_trait]
impl CliCommand for Setup {
    type Output = ();

    // Output the counterfactual address
    async fn run(&self) -> Result<Self::Output> {
        let transaction = OZAccountFactory::deploy(
            &self.private_key_env,
            self.salt.unwrap_or(0),
            self.max_fee.unwrap_or(0),
            &self.network,
        )
        .await?;

        dbg!(transaction);

        Ok(())
    }
}
