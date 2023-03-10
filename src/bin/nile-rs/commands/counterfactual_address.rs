use std::str::FromStr;

use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use clap::Parser;
use starknet_core::types::{contract::legacy::LegacyContractClass, FieldElement};
use starknet_crypto::get_public_key;

use super::CliCommand;
use nile_rs::common::artifacts::Account;
use nile_rs::utils::compute_contract_address;

#[derive(Parser, Debug)]
pub struct CounterfactualAddress {
    #[clap(
        help = "Environment variable set to the private key",
        value_name = "PRIVATE_KEY_ENV"
    )]
    pub private_key_env: String,

    #[clap(help = "Salt for the address generation", long, short)]
    pub salt: Option<u32>,
}

#[async_trait]
impl CliCommand for CounterfactualAddress {
    type Output = ();

    // Output the counterfactual address
    async fn run(&self) -> Result<Self::Output> {
        // Get Private Key from environemnt
        let private_key = std::env::var(&self.private_key_env).with_context(|| {
            format!(
                "Failed to read the private key from `{}`",
                &self.private_key_env
            )
        })?;

        // Get Public Key from Private Key
        let public_key = get_public_key(&FieldElement::from_str(&private_key)?);

        let mut salt: u32 = 0;
        if let Some(s) = self.salt {
            salt = s;
        };

        // Get Account contract class
        let contract_artifact: LegacyContractClass = serde_json::from_str(Account)?;
        let class_hash = contract_artifact.class_hash()?;

        let address = compute_contract_address(FieldElement::from(salt), class_hash, &[public_key]);

        println!("\nThe counterfactual address is: {:#064x}", address);

        Ok(())
    }
}

#[tokio::test]
async fn pk_env_required() {
    let command = CounterfactualAddress {
        private_key_env: "NOT_SET".into(),
        salt: None,
    };

    let error = command.run().await.unwrap_err();
    // Check top error or context
    assert_eq!(
        format!("{}", error),
        format!("Failed to read the private key from `NOT_SET`",)
    );
}
