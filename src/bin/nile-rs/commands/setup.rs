use anyhow::{Ok, Result};
use async_trait::async_trait;
use clap::Parser;

use super::CliCommand;
use nile_rs::core::accounts::OZAccountFactory;
use nile_rs::core::Deployments;

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
    pub max_fee: Option<u64>,

    #[clap(
        help = "Query the estimated fee for the transaction execution",
        long,
        short,
        default_value_t = false
    )]
    pub estimate_fee: bool,

    #[clap(from_global)]
    network: String,
}

#[async_trait]
impl CliCommand for Setup {
    type Output = ();

    // Setup the Account
    async fn run(&self) -> Result<Self::Output> {
        let factory = OZAccountFactory::new(&self.private_key_env, &self.network).await?;
        let mut deployment = factory.deploy(self.salt.unwrap_or(0));
        let max_fee = self.max_fee.unwrap_or(0);

        if self.estimate_fee {
            let fee = deployment.estimate_fee().await?;

            println!("\nOverall fee: {}", fee.overall_fee);
            println!("Gas price (Wei): {}", fee.gas_price);
            println!("Gas usage: {}", fee.gas_usage);

            Ok(())
        } else {
            if max_fee > 0 {
                deployment = deployment.max_fee(max_fee.into());
            }
            let transaction = deployment.send().await?;

            let address = transaction.address.unwrap();

            // Save the account in deployments
            Deployments::save_account(
                &self.private_key_env,
                &format!("{:#064x}", &address),
                &format!("{:#064x}", factory.public_key.scalar()),
                &self.network,
            )?;

            println!("⏳ Transaction successfully sent!");
            println!();
            println!("Transaction hash: {:#064x}", transaction.transaction_hash);
            println!("Counterfactual address: {:#064x}", address);

            Ok(())
        }
    }
}
