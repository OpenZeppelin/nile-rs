pub mod artifacts;
mod constants;
pub mod legacy;
pub use constants::*;

use anyhow::{Ok, Result};
use starknet_crypto::FieldElement;
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

use crate::config::{types::Network, Config};
use crate::utils::{num_str_to_felt, short_str_to_felt};

pub fn is_number(s: &str) -> bool {
    is_hex(s) || is_decimal(s)
}

pub fn is_decimal(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}

pub fn is_hex(s: &str) -> bool {
    if let Some(stripped) = s.strip_prefix("0x") {
        for c in stripped.chars() {
            if !c.is_ascii_hexdigit() {
                return false;
            }
        }
        true
    } else {
        false
    }
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

pub fn get_network_provider_and_signer(
    private_key: &str,
    network: &str,
) -> Result<(Network, SequencerGatewayProvider, LocalWallet)> {
    let network = Config::get_network(network)?;
    let provider = SequencerGatewayProvider::new(
        Url::parse(&network.gateway)?,
        Url::parse(&network.normalized_feeder_gateway())?,
    );
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(num_str_to_felt(
        private_key,
    )?));

    Ok((network, provider, signer))
}
