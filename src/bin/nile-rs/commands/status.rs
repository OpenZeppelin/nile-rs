use anyhow::{Ok, Result};
use async_trait::async_trait;
use clap::Parser;
use starknet_core::types::TransactionStatus;
use starknet_providers::Provider;
use std::{thread, time};

use super::CliCommand;
use nile_rs::common::get_network_and_provider;
use nile_rs::config::Config;
use nile_rs::utils::num_str_to_felt;

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
        let (_, provider) = get_network_and_provider(&self.network)?;

        let config = Config::get()?;
        let wait_time = time::Duration::from_secs(config.track_interval.into());

        loop {
            println!("Querying the status...");

            let status_info = provider
                .get_transaction_status(num_str_to_felt(&self.tx_hash)?)
                .await?;

            match status_info.status {
                TransactionStatus::Rejected => {
                    println!("❌ Transaction status: {:?}", status_info.status);
                    break;
                }
                TransactionStatus::AcceptedOnL1 | TransactionStatus::AcceptedOnL2 => {
                    println!("✅ Transaction status: {:?}", status_info.status);
                    break;
                }
                _ => {
                    println!("⏳ Transaction status: {:?}", status_info.status);

                    if self.track {
                        println!("\nRetrying in {} seconds...", wait_time.as_secs());
                        thread::sleep(wait_time)
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
