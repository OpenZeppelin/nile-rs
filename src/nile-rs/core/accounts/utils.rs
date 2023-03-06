use anyhow::{Ok, Result};
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

use crate::common::str_to_felt;
use crate::config::{types::Network, Config};

pub fn get_network_provider_and_signer(
    private_key: &str,
    network: &str,
) -> Result<(Network, SequencerGatewayProvider, LocalWallet)> {
    let network = Config::get_network(network)?;
    let provider = SequencerGatewayProvider::new(
        Url::parse(&network.gateway)?,
        Url::parse(&network.normalized_feeder_gateway())?,
    );
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(str_to_felt(private_key)));

    Ok((network, provider, signer))
}
