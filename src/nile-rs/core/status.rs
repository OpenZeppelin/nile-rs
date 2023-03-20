use anyhow::{Ok, Result};
use starknet_core::types::{TransactionStatus, TransactionStatusInfo};
use starknet_providers::Provider;
use std::{thread, time};

use crate::common::get_network_and_provider;
use crate::config::Config;
use crate::utils::num_str_to_felt;

pub async fn get_tx_status(
    tx_hash: &str,
    network: &str,
    track: bool,
) -> Result<TransactionStatusInfo> {
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
                break Ok(status_info);
            }
            TransactionStatus::AcceptedOnL1 | TransactionStatus::AcceptedOnL2 => {
                println!("✅ Transaction status: {:?}", status_info.status);
                break Ok(status_info);
            }
            _ => {
                println!("⏳ Transaction status: {:?}", status_info.status);

                if track {
                    println!("\nRetrying in {} seconds...", wait_time.as_secs());
                    thread::sleep(wait_time)
                } else {
                    break Ok(status_info);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::get_tx_status;
    use nile_test_utils::{clean_env, mock_network};

    use httpmock::prelude::*;
    use serde_json::json;

    fn mock_get_status_endpoint(server: MockServer) {
        server.mock(|when, then| {
            when.path("/feeder_gateway/get_transaction_status");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!(
                { "tx_status": "NOT_RECEIVED" }));
        });
    }

    #[tokio::test]
    async fn not_received() {
        let server = MockServer::start();
        let network = "local_test";
        mock_network(network, &server.url("/gateway"));
        mock_get_status_endpoint(server);

        let status = get_tx_status("0x1234", network, false).await.unwrap();
        assert_eq!(
            status.status,
            starknet_core::types::TransactionStatus::NotReceived
        );

        // Clean env after finishing using the mocked network
        clean_env()
    }

    #[tokio::test]
    async fn pending() {
        let server = MockServer::start();
        let network = "local_test";
        mock_network(network, &server.url("/gateway"));

        server.mock(|when, then| {
            when.path("/feeder_gateway/get_transaction_status");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!(
                { "tx_status": "PENDING" }));
        });

        let status = get_tx_status("0x1234", network, false).await.unwrap();
        assert_eq!(
            status.status,
            starknet_core::types::TransactionStatus::Pending
        );

        // Clean env after finishing using the mocked network
        clean_env()
    }

    #[tokio::test]
    async fn rejected() {
        let server = MockServer::start();
        let network = "local_test";
        mock_network(network, &server.url("/gateway"));

        server.mock(|when, then| {
            when.path("/feeder_gateway/get_transaction_status");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!(
                { "tx_status": "REJECTED",
                  "tx_failure_reason" : {"code": "1", "error_message": "reason"}}));
        });

        let status = get_tx_status("0x1234", network, false).await.unwrap();
        assert_eq!(
            status.status,
            starknet_core::types::TransactionStatus::Rejected
        );

        // Clean env after finishing using the mocked network
        clean_env()
    }
}
