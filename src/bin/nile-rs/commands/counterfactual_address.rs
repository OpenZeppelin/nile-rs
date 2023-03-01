use std::str::FromStr;

use anyhow::{Context, Ok, Result};
use clap::Parser;
use starknet_core::types::{ContractArtifact, FieldElement};
use starknet_crypto::get_public_key;

use super::CliCommand;
use nile_rs::common::artifacts::Account;
use nile_rs::utils::compute_contract_address;

#[derive(Parser, Debug)]
pub struct CounterfactualAddress {
    #[clap(
        help = "Environment variable set to the private key.",
        value_name = "PRIVATE_KEY_ENV"
    )]
    pub private_key_env: String,

    #[clap(help = "Salt for the address generation.", value_name = "SALT")]
    pub salt: Option<u32>,
}

impl CliCommand for CounterfactualAddress {
    type Output = ();

    // Output the counterfactual address
    fn run(self) -> Result<Self::Output> {
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
        let contract_artifact: ContractArtifact = serde_json::from_str(Account)?;
        let class_hash = contract_artifact.class_hash()?;

        let address = compute_contract_address(FieldElement::from(salt), class_hash, &[public_key]);

        println!("\nThe counterfactual address is: {:#064x}", address);

        Ok(())
    }
}

#[test]
fn test_pk_env_required() {
    let command = CounterfactualAddress {
        private_key_env: "NOT_SET".into(),
        salt: None,
    };

    let error = command.run().unwrap_err();
    // Check top error or context
    assert_eq!(
        format!("{}", error),
        format!("Failed to read the private key from `NOT_SET`",)
    );
}
