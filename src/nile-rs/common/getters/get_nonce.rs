use anyhow::{Ok, Result};
use starknet_core::types::BlockId;
use starknet_crypto::FieldElement;
use starknet_providers::Provider;

use crate::{common::get_network_and_provider, utils::num_str_to_felt};

pub async fn get_nonce(address: &str, network: &str) -> Result<FieldElement> {
    let address = num_str_to_felt(address)?;
    let (_, provider) = get_network_and_provider(network)?;
    let nonce = provider.get_nonce(address, BlockId::Latest).await?;
    Ok(nonce)
}
