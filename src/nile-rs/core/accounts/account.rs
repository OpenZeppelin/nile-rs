use anyhow::{Context, Ok, Result};
use starknet_accounts::{Account, Declaration, SingleOwnerAccount};
use starknet_core::types::contract::legacy::LegacyContractClass;
use starknet_crypto::FieldElement;
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::LocalWallet;
use std::fmt::Debug;
use std::sync::Arc;

use super::db::DB;
use super::utils::get_network_provider_and_signer;
use crate::config::Config;

pub struct OZAccount {
    inner: SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>,
}

impl Debug for OZAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.inner.address()).finish()
    }
}

/// Wrapper for the starknet-rs SingleOwnerAccount
impl OZAccount {
    pub fn new(private_key_env: &str, network: &str) -> Result<Self> {
        // Get the signer's private key
        let private_key = std::env::var(private_key_env).with_context(|| {
            format!("Failed to read the private key from `{}`", private_key_env)
        })?;

        let address = DB::load_from_signer(private_key_env, network)?.address;
        // Get network, provider and signer
        let (network, provider, signer) = get_network_provider_and_signer(&private_key, network)?;

        Ok(Self {
            inner: SingleOwnerAccount::new(
                provider,
                signer,
                FieldElement::from_hex_be(&address)?,
                network.chain_id_in_felt(),
            ),
        })
    }

    pub fn new_with_private_key(private_key: &str, address: &str, network: &str) -> Result<Self> {
        // Get network, provider and signer
        let (network, provider, signer) = get_network_provider_and_signer(private_key, network)?;

        Ok(Self {
            inner: SingleOwnerAccount::new(
                provider,
                signer,
                FieldElement::from_hex_be(address)?,
                network.chain_id_in_felt(),
            ),
        })
    }

    /// Declare Cairo 0 artifacts
    pub fn legacy_declare(
        &self,
        contract_name: &str,
    ) -> Result<Declaration<SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>>> {
        let config = Config::get()?;
        let path = [&config.artifacts_dir, "/", contract_name, ".json"].concat();

        let context = || {
            format!(
                "Failed to read the artifact from: `{}`",
                path.replace("//", "/")
            )
        };
        let contract_artifact: LegacyContractClass =
            serde_json::from_reader(std::fs::File::open(&path).with_context(context)?)
                .with_context(context)?;

        let declaration = self.inner.declare(Arc::new(contract_artifact));
        Ok(declaration)
    }
}
