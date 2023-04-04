use anyhow::Result;

use async_trait::async_trait;
use clap::Parser;
use nile_rs::common::getters::get_balance;

use super::CliCommand;

#[derive(Parser, Debug)]
pub struct GetBalance {
    #[clap(help = "The contract address")]
    pub address: String,

    #[clap(from_global)]
    network: String,
}

#[async_trait]
impl CliCommand for GetBalance {
    type Output = ();

    // Get the balance from the provided address
    async fn run(&self) -> Result<Self::Output> {
        let balance = get_balance(&self.address, &self.network).await?;

        let small: u32 = 10 ^ 6;
        let medium: u32 = 10 ^ 15;

        println!("The current balance is:");
        if balance < small.into() {
            println!("🪙 {:#} wei", balance);
        } else if balance < medium.into() {
            println!("💰 {:#} gwei", balance.to_big_decimal(9));
        } else {
            println!("🤑 {:#} ether", balance.to_big_decimal(18));
        }

        Ok(())
    }
}
