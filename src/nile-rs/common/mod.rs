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

#[test]
fn is_decimal_output() {
    assert!(!is_decimal("0x123"));
    assert!(!is_decimal("abc"));
    assert!(is_decimal("123"));
    assert!(!is_decimal("123k"));
}

#[test]
fn is_hex_output() {
    assert!(is_hex("0x123"));
    assert!(is_hex("0xabc"));
    assert!(!is_hex("123"));
    assert!(!is_hex("0xk"));
}

#[test]
fn normalize_calldata_output() {
    let calldata: Vec<String> = vec!["123".into(), "TOKEN".into()];
    let normalized = normalize_calldata(calldata);

    assert_eq!(normalized[0], num_str_to_felt("123").unwrap());
    assert_eq!(normalized[1], short_str_to_felt("TOKEN").unwrap());
}
