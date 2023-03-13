use anyhow::{Context, Ok, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json;
use url::Url;

use crate::config::Config;
use crate::core::accounts::OZAccount;

#[derive(Deserialize)]
struct PredeployedAccount {
    private_key: String,
    address: String,
}

pub async fn get_predeployed_accounts(network: &str) -> Result<Vec<OZAccount>> {
    let network_str = network;
    let network = Config::get_network(network)?;

    // Get accounts from http request
    let endpoint = network.predeployed_accounts_endpoint();
    let res = Client::new()
        .get(Url::parse(&endpoint)?)
        .send()
        .await
        .with_context(|| format!("Failed to get the accounts from `{}`", &endpoint))?;

    let body = res.text().await?;

    let predeployed_accounts: Vec<PredeployedAccount> = serde_json::from_str(&body)?;
    let oz_accounts: Vec<OZAccount> = predeployed_accounts
        .iter()
        .map(|pa| {
            OZAccount::new_with_private_key(&pa.private_key, &pa.address, network_str).unwrap()
        })
        .collect();

    Ok(oz_accounts)
}

#[cfg(test)]
mod test {
    use super::get_predeployed_accounts;
    use nile_test_utils::{clean_env, mock_network};

    use httpmock::prelude::*;
    use serde_json::json;

    #[tokio::test]
    async fn endpoint_error() {
        let error = get_predeployed_accounts("localhost").await.unwrap_err();

        // Check top error or context
        assert_eq!(
            format!("{}", error),
            format!("Failed to get the accounts from `http://127.0.0.1:5050/predeployed_accounts`",)
        );
    }

    #[tokio::test]
    async fn return_value() {
        let server = MockServer::start();
        let network = "local_test";
        mock_network(network, &server.url("/gateway"));

        server.mock(|when, then| {
            when.path("/predeployed_accounts");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!([
                { "private_key": "0x1", "address": "0x2" },
                { "private_key": "0x3", "address": "0x4" }]));
        });

        let accounts = get_predeployed_accounts(network).await.unwrap();
        assert_eq!(accounts.len(), 2);

        // Clean env after finishing using the mocked network
        clean_env()
    }
}
