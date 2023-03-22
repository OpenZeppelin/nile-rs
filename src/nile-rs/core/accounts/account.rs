use anyhow::{Context, Ok, Result};
use starknet_accounts::{Account, Call, Declaration, Execution, SingleOwnerAccount};
use starknet_core::utils::get_selector_from_name;
use starknet_crypto::FieldElement;
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::LocalWallet;
use std::fmt::Debug;
use std::sync::Arc;

use crate::core::Deployments;
use crate::utils::num_str_to_felt;
use crate::{
    common::{
        get_network_provider_and_signer,
        legacy::{get_legacy_class_hash, get_legacy_contract_class},
        normalize_calldata, SELECTOR_DEPLOYCONTRACT, UDC_ADDRESS,
    },
    utils::udc_deployment_address,
};

pub struct OZAccount {
    inner: SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>,
    pub address: FieldElement,
}

impl Debug for OZAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("address").field(&self.inner.address().to_string()).finish()
    }
}

/// Wrapper for the starknet-rs SingleOwnerAccount
impl OZAccount {
    pub fn new(private_key_env: &str, network: &str) -> Result<Self> {
        // Get the signer's private key
        let private_key = std::env::var(private_key_env).with_context(|| {
            format!("Failed to read the private key from `{}`", private_key_env)
        })?;

        let address = Deployments::load_account(private_key_env, network)?.address;
        // Get network, provider and signer
        let (network, provider, signer) = get_network_provider_and_signer(&private_key, network)?;
        let address = num_str_to_felt(&address)?;

        Ok(Self {
            inner: SingleOwnerAccount::new(provider, signer, address, network.chain_id_in_felt()),
            address,
        })
    }

    pub fn new_with_private_key(private_key: &str, address: &str, network: &str) -> Result<Self> {
        // Get network, provider and signer
        let (network, provider, signer) = get_network_provider_and_signer(private_key, network)?;
        let address = num_str_to_felt(address)?;

        Ok(Self {
            inner: SingleOwnerAccount::new(provider, signer, address, network.chain_id_in_felt()),
            address,
        })
    }

    /// Declare Cairo 0 artifacts
    pub fn legacy_declare(
        &self,
        contract_name: &str,
    ) -> Result<Declaration<SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>>> {
        let contract_artifact = get_legacy_contract_class(contract_name)?;

        let declaration = self.inner.declare(Arc::new(contract_artifact));
        Ok(declaration)
    }

    /// Execute transactions
    pub fn execute(
        &self,
        contract: &str,
        method: &str,
        calldata: Vec<String>,
    ) -> Result<Execution<SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>>> {
        let execution = self.inner.execute(vec![Call {
            to: num_str_to_felt(contract)?,
            selector: get_selector_from_name(method)?,
            calldata: normalize_calldata(calldata),
        }]);
        Ok(execution)
    }

    /// Deploy contracts through UDC
    pub fn deploy(
        &self,
        contract_name: &str,
        salt: u32,
        unique: bool,
        calldata: Vec<String>,
    ) -> Result<(
        Execution<SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>>,
        FieldElement,
    )> {
        let class_hash = get_legacy_class_hash(contract_name)?;
        let constructor_calldata = normalize_calldata(calldata);

        let mut calldata = vec![
            class_hash,
            salt.into(),
            if unique {
                FieldElement::ONE
            } else {
                FieldElement::ZERO
            },
            constructor_calldata.len().into(),
        ];
        constructor_calldata
            .iter()
            .for_each(|item| calldata.push(*item));

        let address = udc_deployment_address(
            class_hash,
            salt.into(),
            unique,
            &constructor_calldata,
            self.address,
        )?;

        Ok((
            Execution::new(
                vec![Call {
                    to: UDC_ADDRESS,
                    selector: SELECTOR_DEPLOYCONTRACT,
                    calldata,
                }],
                &self.inner,
            ),
            address,
        ))
    }
}
