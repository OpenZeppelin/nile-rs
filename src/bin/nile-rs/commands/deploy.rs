use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use clap::Parser;

use super::CliCommand;
use nile_rs::common::devnet::get_predeployed_accounts;
use nile_rs::core::accounts::OZAccount;
use nile_rs::core::status::get_tx_status;
use nile_rs::core::Deployments;

#[derive(Parser, Debug)]
#[command(group(
  clap::ArgGroup::new("account")
      .required(true)
      .args(&["private_key_env", "devnet_account"]),
))]
pub struct Deploy {
    #[clap(
        short,
        long,
        help = "Environment variable set to the private key",
        value_name = "ENV"
    )]
    pub private_key_env: Option<String>,

    #[clap(
        short,
        long,
        help = "Index of the predeployed account from devnet",
        value_name = "INDEX"
    )]
    pub devnet_account: Option<usize>,

    #[clap(help = "The contract to deploy")]
    pub contract_name: String,

    #[clap(help = "The constructor calldata")]
    pub parameters: Option<Vec<String>>,

    #[clap(short, long, help = "Alias for referring to")]
    pub alias: Option<String>,

    #[clap(
        short,
        long,
        help = "The salt for the UDC address generation",
        default_value_t = 0
    )]
    pub salt: u32,

    #[clap(
        help = "Flag as unique for the UDC address generation",
        short,
        long,
        default_value_t = false
    )]
    pub unique: bool,

    #[clap(help = "Max fee allowed to pay for the transaction", long, short)]
    pub max_fee: Option<u64>,

    #[clap(
        help = "Query the estimated fee for the deployment execution",
        short,
        long,
        default_value_t = false
    )]
    pub estimate_fee: bool,

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

impl Deploy {
    async fn run_with_account(&self, account: &OZAccount) -> Result<()> {
        let mut params = vec![];
        if let Some(vector) = self.parameters.clone() {
            params = vector;
        }
        let (mut deployment, address) =
            account.deploy(&self.contract_name, self.salt, self.unique, params)?;
        let max_fee = self.max_fee.unwrap_or(0);

        if self.estimate_fee {
            let fee = deployment.estimate_fee().await?;

            println!("\nOverall fee: {}", fee.overall_fee);
            println!("Gas price (Wei): {}", fee.gas_price);
            println!("Gas usage: {}", fee.gas_usage);
        } else {
            if max_fee > 0 {
                deployment = deployment.max_fee(max_fee.into());
            }
            let transaction = deployment
                .send()
                .await
                .with_context(|| "Failed attempt to send the deployment")?;

            // Save the contract in deployments
            Deployments::save_contract(
                self.alias.clone(),
                &format!("{:#064x}", &address),
                &self.network,
            )?;

            let tx_hash = format!("{:#064x}", transaction.transaction_hash);

            println!("⏳ Deployment successfully sent!");
            println!();
            println!("Transaction hash: {}", &tx_hash);
            println!("Contract address: {:#064x}", address);

            if self.track {
                get_tx_status(&tx_hash, &self.network, self.track).await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl CliCommand for Deploy {
    type Output = ();

    // Execute a transaction through an Account
    async fn run(&self) -> Result<Self::Output> {
        if self.devnet_account.is_some() {
            let predeployed_accounts = get_predeployed_accounts(&self.network).await?;
            let account = &predeployed_accounts[self.devnet_account.unwrap()];

            self.run_with_account(account).await
        } else {
            let account = &OZAccount::new(self.private_key_env.as_ref().unwrap(), &self.network)?;

            self.run_with_account(account).await
        }
    }
}
