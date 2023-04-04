use anyhow::Result;

use async_trait::async_trait;
use clap::Parser;
use nile_rs::common::getters::get_accounts;

use super::CliCommand;

#[derive(Parser, Debug)]
pub struct GetAccounts {
    #[clap(
        short,
        long,
        help = "Query the predeployed accounts (devnet only)",
        default_value_t = false
    )]
    pub predeployed_accounts: bool,

    #[clap(from_global)]
    network: String,
}

#[async_trait]
impl CliCommand for GetAccounts {
    type Output = ();

    // Get the accounts registered for the given network
    async fn run(&self) -> Result<Self::Output> {
        let accounts = get_accounts(&self.network, self.predeployed_accounts).await?;

        println!();
        for (i, account) in accounts.iter().enumerate() {
          println!("Account {}: {:#064x}", i, account.address)
        };

        Ok(())
    }
}
