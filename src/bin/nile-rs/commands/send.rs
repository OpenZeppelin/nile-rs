use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use clap::Parser;
use nile_rs::core::accounts::OZAccount;
use nile_rs::utils::devnet::get_predeployed_accounts;

use super::CliCommand;
use nile_rs::core::Deployments;

#[derive(Parser, Debug)]
#[command(group(
  clap::ArgGroup::new("account")
      .required(true)
      .args(&["private_key_env", "devnet_account"]),
),
group(
    clap::ArgGroup::new("contract_id")
        .required(true)
        .args(&["address", "alias"]),
  ))]
pub struct Send {
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

    #[clap(
        long,
        help = "The address of the contract to call",
        value_name = "ADDRESS"
    )]
    pub address: Option<String>,

    #[clap(long, help = "The alias of the contract to call", value_name = "ALIAS")]
    pub alias: Option<String>,

    #[clap(help = "The method to execute")]
    pub method: String,

    #[clap(help = "The calldata")]
    pub parameters: Option<Vec<String>>,

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

impl Send {
    async fn run_with_account(&self, account: &OZAccount) -> Result<()> {
        // Define params if empty
        let mut params = vec![];
        if let Some(vector) = self.parameters.clone() {
            params = vector;
        };

        // Get contract by alias or address
        let address;
        if let Some(alias) = &self.alias {
            address = Deployments::load_contract_from_alias(alias, &self.network)?.address;
        } else {
            address = Deployments::load_contract_from_address(
                self.address.as_ref().unwrap(),
                &self.network,
            )?
            .address;
        }

        let mut execution = account.execute(&address, &self.method, params)?;
        let max_fee = self.max_fee.unwrap_or(0);

        if self.estimate_fee {
            let fee = execution.estimate_fee().await?;

            println!("\nOverall fee: {}", fee.overall_fee);
            println!("Gas price (Wei): {}", fee.gas_price);
            println!("Gas usage: {}", fee.gas_usage);
        } else {
            if max_fee > 0 {
                execution = execution.max_fee(max_fee.into());
            }
            let transaction = execution
                .send()
                .await
                .with_context(|| "Failed attempt to send the transaction")?;

            println!("⏳ Transaction successfully sent!");
            println!();
            println!("Transaction hash: {:#064x}", transaction.transaction_hash);
        }
        Ok(())
    }
}

#[async_trait]
impl CliCommand for Send {
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
