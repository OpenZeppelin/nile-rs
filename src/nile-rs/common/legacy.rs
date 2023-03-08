use anyhow::{Context, Result};
use starknet_core::types::contract::legacy::LegacyContractClass;
use starknet_crypto::FieldElement;

use crate::config::Config;

pub fn get_legacy_contract_class(contract_name: &str) -> Result<LegacyContractClass> {
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

pub fn get_legacy_class_hash(contract_name: &str) -> Result<FieldElement> {
    let contract_artifact = get_legacy_contract_class(contract_name)?;
    contract_artifact
        .class_hash()
        .with_context(|| "Failed to obtain the class hash")
}
