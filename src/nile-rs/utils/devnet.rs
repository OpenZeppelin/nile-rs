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

    // Get account from http request
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
