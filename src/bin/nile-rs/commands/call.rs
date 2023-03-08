use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use clap::Parser;
use starknet_core::types::{BlockId, CallFunction};
use starknet_core::utils::get_selector_from_name;
use starknet_providers::Provider;

use super::CliCommand;
use nile_rs::common::{get_network_provider_and_signer, normalize_calldata};
use nile_rs::utils::num_str_to_felt;

#[derive(Parser, Debug)]
pub struct Call {
    #[clap(help = "The contract to call", value_name = "ADDRESS")]
    pub contract: String,

    #[clap(help = "The method to execute")]
    pub method: String,

    #[clap(help = "The calldata")]
    pub parameters: Option<Vec<String>>,

    #[clap(from_global)]
    network: String,
}

#[async_trait]
impl CliCommand for Call {
    type Output = ();

    // Query the blockchain
    async fn run(&self) -> Result<Self::Output> {
        let mut params = vec![];
        if let Some(vector) = self.parameters.clone() {
            params = vector;
        }
        let (_, provider, _) = get_network_provider_and_signer("0", &self.network)?;

        let call_result = provider
            .call_contract(
                CallFunction {
                    contract_address: num_str_to_felt(&self.contract)?,
                    entry_point_selector: get_selector_from_name(&self.method)?,
                    calldata: normalize_calldata(params),
                },
                BlockId::Latest,
            )
            .await
            .with_context(|| "Failed to query the blockchain")?;

        dbg!(call_result);
        Ok(())
    }
}
