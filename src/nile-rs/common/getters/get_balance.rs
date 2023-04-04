use anyhow::{Ok, Result};
use starknet_core::{types::BlockId, utils::get_storage_var_address};
use starknet_crypto::FieldElement;
use starknet_providers::Provider;

use crate::{common::get_network_and_provider, utils::num_str_to_felt};

pub async fn get_balance(address: &str, network: &str) -> Result<FieldElement> {
    let address = num_str_to_felt(address)?;
    let (_, provider) = get_network_and_provider(network)?;

    // Checks L2 ETH balance via storage taking advantage of implementation detail
    let eth_balance = provider
        .get_storage_at(
            FieldElement::from_hex_be(
                "049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
            )
            .unwrap(),
            get_storage_var_address("ERC20_balances", &[address]).unwrap(),
            BlockId::Latest,
        )
        .await
        .unwrap();
    Ok(eth_balance)
}
