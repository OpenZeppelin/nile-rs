use anyhow::{anyhow, Context, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::utils::num_str_to_felt;

const TO_REPLACE: &str = "<network>";
const FILE_NAME_FORMAT: &str = "<network>.contracts.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractInfo {
    pub alias: String,
    pub address: String,
}

impl ContractInfo {
    /// Attempt to get the contract data from the alias
    pub fn load_from_alias(alias: &str, network: &str) -> Result<ContractInfo> {
        let contracts = Self::get_contracts(network)?;

        let result = contracts
            .into_iter()
            .find(|contract| contract.alias == alias);

        match result {
            Some(contract) => Ok(contract),
            None => Err(anyhow!(
                "Contract not found! If the contract is deployed \
                already, try registering it in `{}`",
                Self::get_db_file_name(network)?.replace("//", "/")
            )),
        }
    }

    /// Attempt to get the contract data from the address
    pub fn load_from_address(address: &str, network: &str) -> Result<ContractInfo> {
        let contracts = Self::get_contracts(network)?;

        let result = contracts.into_iter().find(|contract| {
            num_str_to_felt(&contract.address).unwrap() == num_str_to_felt(address).unwrap()
        });

        match result {
            Some(contract) => Ok(contract),
            None => Err(anyhow!(
                "Contract not found! If the contract is deployed \
                already, try registering it in `{}`",
                Self::get_db_file_name(network)?.replace("//", "/")
            )),
        }
    }

    fn get_contracts(network: &str) -> Result<Vec<ContractInfo>> {
        let db_file_name = Self::get_db_file_name(network)?;

        let context = || {
            format!(
                "Failed to load the contract from: `{}`",
                db_file_name.replace("//", "/")
            )
        };
        let contracts: Vec<ContractInfo> =
            serde_json::from_reader(std::fs::File::open(&db_file_name).with_context(context)?)
                .with_context(context)?;

        Ok(contracts)
    }

    fn get_db_file_name(network: &str) -> Result<String> {
        let config = Config::get()?;
        Ok([
            &config.deployments_dir,
            "/",
            &FILE_NAME_FORMAT.replace(TO_REPLACE, network),
        ]
        .concat())
    }

    /// Attempt to save the contract data in the file system
    pub fn save(alias: &str, address: &str, network: &str) -> Result<()> {
        let config = Config::get()?;
        let db_file_name = [
            &config.deployments_dir,
            "/",
            &FILE_NAME_FORMAT.replace(TO_REPLACE, network),
        ]
        .concat();

        // Ensure the directories exists
        std::fs::create_dir_all(&config.deployments_dir)?;

        let new_contract = ContractInfo {
            alias: alias.into(),
            address: address.into(),
        };

        let mut contracts: Vec<ContractInfo> = vec![];
        if std::path::Path::new(&db_file_name).exists() {
            contracts = serde_json::from_reader(std::fs::File::open(&db_file_name)?).with_context(
                || {
                    format!(
                        "Failed to load the existing contracts from `{}`.\n\
                        Try removing the file if it is empty.",
                        db_file_name.replace("//", "/")
                    )
                },
            )?;
        };

        contracts.push(new_contract);

        // Save the JSON structure into the file.
        std::fs::write(
            db_file_name,
            serde_json::to_string_pretty(&contracts).unwrap(),
        )?;

        Ok(())
    }
}

#[test]
fn error_context() {
    let error = ContractInfo::load_from_address("0x1", "test").unwrap_err();
    assert_eq!(
        format!("{}", error),
        "Failed to load the contract from: `deployments/test.contracts.json`"
    );
}
