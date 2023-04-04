use crate::{
    common::{
        devnet::get_predeployed_accounts,
        get_registered_accounts,
        getters::{get_balance, get_nonce},
    },
    config::Config,
    core::{accounts::OZAccount, types::Network},
};
use anyhow::{Ok, Result};
use starknet_crypto::FieldElement;

pub struct NileRuntimeEnvironment {
    pub network: Network,
}

impl NileRuntimeEnvironment {
    pub fn new(network: &str) -> Result<Self> {
        let network = Config::get_network(network)?;

        Ok(Self { network })
    }

    pub fn get_accounts(&self) -> Result<Vec<OZAccount>> {
        get_registered_accounts(&self.network.name)
    }

    pub async fn get_predeployed_accounts(&self) -> Result<Vec<OZAccount>> {
        get_predeployed_accounts(&self.network.name).await
    }

    pub async fn get_nonce(&self, address: &str) -> Result<FieldElement> {
        get_nonce(address, &self.network.name).await
    }

    pub async fn get_balance(&self, address: &str) -> Result<FieldElement> {
        get_balance(address, &self.network.name).await
    }
}
