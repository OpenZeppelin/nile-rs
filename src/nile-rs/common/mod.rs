pub mod artifacts;
mod constants;
pub mod devnet;
pub mod legacy;
pub mod types;
pub use constants::*;

use anyhow::{Context, Ok, Result};
use starknet_crypto::FieldElement;
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

use crate::config::Config;
use crate::core::accounts::OZAccount;
use crate::core::deployments::AccountInfo;
use crate::utils::{is_number, num_str_to_felt, short_str_to_felt};
use types::Network;

pub fn get_accounts(network: &str) -> Result<Vec<OZAccount>> {
    let accounts: Vec<AccountInfo> = AccountInfo::load_all(network)?;

    let oz_accounts: Vec<OZAccount> = accounts
        .iter()
        .map(|acc_info| {
            let private_key = std::env::var(&acc_info.name)
                .with_context(|| {
                    format!("Failed to read the private key from `{}`", &acc_info.name)
                })
                .unwrap();
            OZAccount::new_with_private_key(&private_key, &acc_info.address, network).unwrap()
        })
        .collect();

    Ok(oz_accounts)
}

pub fn get_network_provider_and_signer(
    private_key: &str,
    network: &str,
) -> Result<(Network, SequencerGatewayProvider, LocalWallet)> {
    let (network, provider) = get_network_and_provider(network)?;
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(num_str_to_felt(
        private_key,
    )?));

    Ok((network, provider, signer))
}

pub fn get_network_and_provider(network: &str) -> Result<(Network, SequencerGatewayProvider)> {
    let network = Config::get_network(network)?;
    let provider = SequencerGatewayProvider::new(
        Url::parse(&network.gateway)?,
        Url::parse(&network.normalized_feeder_gateway())?,
    );
    Ok((network, provider))
}

pub fn normalize_calldata(calldata: Vec<String>) -> Vec<FieldElement> {
    let mut vector = Vec::new();
    for param in &calldata {
        if is_number(param) {
            vector.push(num_str_to_felt(param).unwrap());
        } else {
            // Assume is short string
            vector.push(short_str_to_felt(param).unwrap());
        }
    }
    vector
}

#[test]
fn normalize_calldata_output() {
    let calldata: Vec<String> = vec!["123".into(), "TOKEN".into()];
    let normalized = normalize_calldata(calldata);

    assert_eq!(normalized[0], num_str_to_felt("123").unwrap());
    assert_eq!(normalized[1], short_str_to_felt("TOKEN").unwrap());
}
