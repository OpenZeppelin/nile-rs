use std::fmt::Debug;

use anyhow::{Context, Ok, Result};
use starknet_accounts::{AccountDeployment, AccountFactory, OpenZeppelinAccountFactory};
use starknet_core::types::contract::legacy::LegacyContractClass;
use starknet_crypto::FieldElement;
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, Signer, VerifyingKey};

use crate::common::{artifacts::Account, get_network_provider_and_signer};

pub struct OZAccountFactory {
    factory: OpenZeppelinAccountFactory<LocalWallet, SequencerGatewayProvider>,
    pub public_key: VerifyingKey,
}

impl Debug for OZAccountFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.public_key).finish()
    }
}

impl OZAccountFactory {
    pub async fn new(private_key_env: &str, network: &str) -> Result<Self> {
        // Get Account contract class
        let contract_artifact: LegacyContractClass = serde_json::from_str(Account)?;
        let class_hash = contract_artifact.class_hash()?;

        // Get the signer's private key
        let private_key = std::env::var(private_key_env).with_context(|| {
            format!("Failed to read the private key from `{}`", private_key_env)
        })?;

        // Get network, provider and signer
        let (network, provider, signer) = get_network_provider_and_signer(&private_key, network)?;

        let factory = OpenZeppelinAccountFactory::new(
            class_hash,
            network.chain_id_in_felt(),
            signer.clone(),
            provider,
        )
        .await?;

        Ok(Self {
            factory,
            public_key: signer.get_public_key().await?,
        })
    }

    /// Execute the deployment
    pub fn deploy(
        &self,
        salt: u32,
    ) -> AccountDeployment<OpenZeppelinAccountFactory<LocalWallet, SequencerGatewayProvider>> {
        self.factory.deploy(salt.into()).nonce(FieldElement::ZERO)
    }
}

#[tokio::test]
async fn pk_env_required() {
    let error = OZAccountFactory::new("NOT_SET", "localhost")
        .await
        .unwrap_err();

    // Check top error or context
    assert_eq!(
        format!("{}", error),
        format!("Failed to read the private key from `NOT_SET`",)
    );
}

#[tokio::test]
async fn valid_network_check() {
    std::env::set_var("SET", "1");
    let error = OZAccountFactory::new("SET", "invalid").await.unwrap_err();

    // Check top error or context
    assert_eq!(format!("{}", error), format!("Network not found!",));
}

#[tokio::test]
async fn auto_fee_estimation_when_zero() {
    std::env::set_var("SET", "1");
    let factory = OZAccountFactory::new("SET", "localhost").await.unwrap();

    let error = factory.deploy(0).send().await.unwrap_err();

    // Check root cause
    assert!(format!("{}", error).starts_with("error sending request for url"));
}
