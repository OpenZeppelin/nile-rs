mod accounts;
mod contracts;

use accounts::AccountInfo;
use anyhow::Result;
use contracts::ContractInfo;

pub struct Deployments {}

impl Deployments {
    pub fn save_contract(alias: Option<String>, address: &str, network: &str) -> Result<()> {
        if let Some(a) = alias {
            ContractInfo::save(&a, address, network)
        } else {
            ContractInfo::save("", address, network)
        }
    }

    pub fn load_contract_from_alias(alias: &str, network: &str) -> Result<ContractInfo> {
        ContractInfo::load_from_alias(alias, network)
    }

    pub fn load_contract_from_address(address: &str, network: &str) -> Result<ContractInfo> {
        ContractInfo::load_from_address(address, network)
    }

    pub fn save_account(
        private_key_env: &str,
        address: &str,
        public_key: &str,
        network: &str,
    ) -> Result<()> {
        AccountInfo::save(private_key_env, address, public_key, network)
    }

    pub fn load_account(private_key_env: &str, network: &str) -> Result<AccountInfo> {
        AccountInfo::load_from_signer(private_key_env, network)
    }
}
