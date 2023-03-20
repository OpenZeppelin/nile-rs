use anyhow::{Ok, Result};
use starknet_core::types::{TransactionStatus, TransactionStatusInfo};
use starknet_providers::Provider;
use std::{thread, time};

use crate::common::get_network_and_provider;
use crate::config::Config;
use crate::utils::num_str_to_felt;

pub async fn get_tx_status(tx_hash: &str, network: &str, track: bool) -> Result<TransactionStatusInfo> {
    let (_, provider) = get_network_and_provider(network)?;

    let config = Config::get()?;
    let wait_time = time::Duration::from_secs(config.track_interval.into());

    loop {
        println!("Querying the status...");

        let status_info = provider
            .get_transaction_status(num_str_to_felt(tx_hash)?)
            .await?;

        match status_info.status {
            TransactionStatus::Rejected => {
                println!("❌ Transaction status: {:?}", status_info.status);
                if let Some(reason) = &status_info.transaction_failure_reason {
                    println!("{:?}", reason)
                }
                break Ok(status_info)
            }
            TransactionStatus::AcceptedOnL1 | TransactionStatus::AcceptedOnL2 => {
                println!("✅ Transaction status: {:?}", status_info.status);
                break Ok(status_info)
            }
            _ => {
                println!("⏳ Transaction status: {:?}", status_info.status);

                if track {
                    println!("\nRetrying in {} seconds...", wait_time.as_secs());
                    thread::sleep(wait_time)
                } else {
                    break Ok(status_info)
                }
            }
        }
    }
  }