use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use clap::Parser;
use nile_rs::common::devnet::get_predeployed_accounts;
use nile_rs::core::accounts::OZAccount;
use nile_rs::core::status::get_tx_status;

use super::CliCommand;

#[derive(Parser, Debug)]
#[command(group(
  clap::ArgGroup::new("account")
      .required(true)
      .args(&["private_key_env", "devnet_account"]),
))]
pub struct DeclareV2 {
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

    #[clap(help = "The contract name", value_name = "CONTRACT")]
    pub contract_name: String,

    #[clap(help = "Max fee allowed to pay for the transaction", long, short)]
    pub max_fee: Option<u64>,

    #[clap(
        help = "Query the estimated fee for the transaction execution",
        long,
        short,
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

impl DeclareV2 {
    async fn run_with_account(&self, account: &OZAccount) -> Result<()> {
        let mut declaration = account.declare(&self.contract_name)?;
        let max_fee = self.max_fee.unwrap_or(0);

        if self.estimate_fee {
            let fee = declaration.estimate_fee().await?;

            println!("\nOverall fee: {}", fee.overall_fee);
            println!("Gas price (Wei): {}", fee.gas_price);
            println!("Gas usage: {}", fee.gas_usage);
        } else {
            if max_fee > 0 {
                declaration = declaration.max_fee(max_fee.into());
            }
            let transaction = declaration
                .send()
                .await
                .with_context(|| "Failed attempt to send the declare transaction")?;

            let class_hash = transaction.class_hash.unwrap();
            let tx_hash = format!("{:#064x}", transaction.transaction_hash);

            println!("⏳ Declaration successfully sent!");
            println!();
            println!("Transaction hash: {}", &tx_hash);
            println!("Class hash: {:#064x}", class_hash);

            if self.track {
                get_tx_status(&tx_hash, &self.network, self.track).await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl CliCommand for DeclareV2 {
    type Output = ();

    // Declare a contract through an Account
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
