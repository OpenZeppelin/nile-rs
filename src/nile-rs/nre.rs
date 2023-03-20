use crate::{
    common::{devnet::get_predeployed_accounts, get_accounts},
    config::Config,
    core::{accounts::OZAccount, types::Network},
};
use anyhow::{Ok, Result};

pub struct NileRuntimeEnvironment {
    pub network: Network,
}

impl NileRuntimeEnvironment {
    pub fn new(network: &str) -> Result<Self> {
        let network = Config::get_network(network)?;

        Ok(Self { network })
    }

    pub fn get_accounts(&self) -> Result<Vec<OZAccount>> {
        get_accounts(&self.network.name)
    }

    pub async fn get_predeployed_accounts(&self) -> Result<Vec<OZAccount>> {
        get_predeployed_accounts(&self.network.name).await
    }
}
