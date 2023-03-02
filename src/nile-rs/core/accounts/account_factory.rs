use anyhow::{anyhow, Context, Ok, Result};
use starknet_accounts::{AccountFactory, OpenZeppelinAccountFactory};
use starknet_core::types::contract::legacy::LegacyContractClass;
use starknet_core::types::AddTransactionResult;
use starknet_providers::SequencerGatewayProvider;
use starknet_signers::{LocalWallet, SigningKey};
use url::Url;

use crate::common::{artifacts::Account, str_to_felt};
use crate::config::Config;

pub struct OZAccountFactory {}

impl OZAccountFactory {
    pub async fn deploy(
        private_key_env: &str,
        salt: u32,
        max_fee: u128,
        network: &str,
    ) -> Result<AddTransactionResult> {
        // Get Account contract class
        let contract_artifact: LegacyContractClass = serde_json::from_str(Account)?;
        let class_hash = contract_artifact.class_hash()?;

        // Get the signer's private key
        let private_key = std::env::var(private_key_env).with_context(|| {
            format!("Failed to read the private key from `{}`", private_key_env)
        })?;

        let network = Config::get_network(network)?;

        // Get provider and signer
        let provider = SequencerGatewayProvider::new(
            Url::parse(&network.gateway)?,
            Url::parse(&network.normalized_feeder_gateway())?,
        );
        let signer = LocalWallet::from(SigningKey::from_secret_scalar(str_to_felt(&private_key)));

        let factory = OpenZeppelinAccountFactory::new(
            class_hash,
            network.chain_id_in_felt(),
            signer,
            provider,
        )
        .await?;

        let deployment = factory.deploy(salt.into());

        let mut est_fee = max_fee;
        if max_fee == 0 {
            est_fee = deployment
                .estimate_fee()
                .await
                .with_context(|| "Failed to estimate the fee for deploying transaction")?
                .overall_fee
                .into();
        }

        println!(
            "Fund at least {} wei to {:#064x}",
            est_fee,
            deployment.address()
        );
        println!("Press ENTER after account is funded to continue deployment...");
        std::io::stdin().read_line(&mut String::new()).unwrap();

        let result = deployment.send().await;
        match result {
            std::result::Result::Ok(tx) => Ok(tx),
            Err(err) => Err(anyhow!("{err}")).with_context(|| "Failed to execute the deployment"),
        }
    }
}

#[tokio::test]
async fn pk_env_required() {
    let error = OZAccountFactory::deploy("NOT_SET", 0, 0, "localhost")
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

    let error = OZAccountFactory::deploy("SET", 0, 0, "invalid")
        .await
        .unwrap_err();
    // Check top error or context
    assert_eq!(format!("{}", error), format!("Network not found!",));
}

#[tokio::test]
async fn auto_fee_estimation_when_zero() {
    std::env::set_var("SET", "1");

    let error = OZAccountFactory::deploy("SET", 0, 0, "localhost")
        .await
        .unwrap_err();
    // Check top error or context
    assert_eq!(
        format!("{}", error),
        format!("Failed to estimate the fee for deploying transaction",)
    );
}
