pub mod artifacts;
mod constants;
pub mod devnet;
pub mod getters;
pub mod legacy;
pub use constants::*;

use anyhow::{Context, Ok, Result};
use cairo_lang_starknet::casm_contract_class::CasmContractClass;
use cairo_lang_starknet::contract_class::ContractClass;
use starknet_core::types::contract::{CompiledClass, SierraClass};
use starknet_crypto::FieldElement;
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

use crate::config::Config;
use crate::core::accounts::OZAccount;
use crate::core::deployments::AccountInfo;
use crate::core::types::Network;
use crate::utils::{is_number, num_str_to_felt, short_str_to_felt};

pub fn get_registered_accounts(network: &str) -> Result<Vec<OZAccount>> {
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

pub fn get_contract_class(contract_name: &str) -> Result<SierraClass> {
    let config = Config::get()?;
    let path = [&config.artifacts_dir, "/", contract_name, ".json"].concat();

    let context = || {
        format!(
            "Failed to read the artifact from: `{}`",
            path.replace("//", "/")
        )
    };
    serde_json::from_reader(std::fs::File::open(&path).with_context(context)?).with_context(context)
}

pub fn get_compiled_class(contract_name: &str) -> Result<CompiledClass> {
    let config = Config::get()?;
    let path = format!("{}/{}.json", &config.artifacts_dir, contract_name);
    let casm_path = std::env::current_dir()
        .unwrap()
        .join(format!("{}/{}.casm", &config.artifacts_dir, contract_name));

    let context = || {
        format!(
            "Failed to read the artifact from: `{}`",
            path.replace("//", "/")
        )
    };
    let deserializing_context = "Deserializating casm contract class failed";

    let contract_class: ContractClass =
        serde_json::from_reader(std::fs::File::open(&path).with_context(context)?)
            .with_context(context)?;

    let casm_contract_class = CasmContractClass::from_contract_class(contract_class, false)
        .with_context(|| "Compilation failed.")?;
    let res = serde_json::to_string_pretty(&casm_contract_class)
        .with_context(|| "Serialization failed.")?;

    std::fs::write(&casm_path, &res).with_context(|| {
        format!(
            "Failed to write Casm output to: {}",
            casm_path.to_str().unwrap()
        )
    })?;

    serde_json::from_str::<CompiledClass>(&res).with_context(|| deserializing_context)
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
