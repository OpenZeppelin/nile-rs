use figment::{
    providers::{Env, Serialized},
    Error, Figment,
};
use serde::{Deserialize, Serialize};

mod default;
mod scarb_provider;

use crate::core::types::Network;
use scarb_provider::ScarbProvider;

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// Directory containing the Cairo contracts
    pub contracts_dir: String,
    /// Directory where artifacts are stored
    pub artifacts_dir: String,
    /// Directory where deployments are stored
    pub deployments_dir: String,
    /// Frequency for querying the status of a transaction
    pub track_interval: u32,
    /// Custom networks
    pub networks: Vec<Network>,
}

impl Config {
    pub fn get() -> Result<Self, Error> {
        Config::figment().extract::<Self>()
    }

    pub fn get_network(name: &str) -> Result<Network, Error> {
        let config = Self::get()?;
        let result = config
            .networks
            .into_iter()
            .find(|network| network.name == name);

        match result {
            Some(network) => Ok(network),
            None => {
                // Query default networks
                let base = Self::base_networks();
                match name {
                    "localhost" => Ok(base[0].clone()),
                    "mainnet" => Ok(base[1].clone()),
                    "goerli" => Ok(base[2].clone()),
                    "goerli2" => Ok(base[3].clone()),
                    _ => Err(Error::from(String::from("Network not found!"))),
                }
            }
        }
    }

    fn base_networks() -> [Network; 4] {
        [
            Network {
                name: "localhost".into(),
                gateway: "http://127.0.0.1:5050/gateway".into(),
                feeder_gateway: None,
                chain_id: "1536727068981429685321".into(),
            },
            Network {
                name: "mainnet".into(),
                gateway: "https://alpha-mainnet.starknet.io/gateway".into(),
                feeder_gateway: None,
                chain_id: "23448594291968334".into(),
            },
            Network {
                name: "goerli".into(),
                gateway: "https://alpha4.starknet.io/gateway".into(),
                feeder_gateway: None,
                chain_id: "1536727068981429685321".into(),
            },
            Network {
                name: "goerli2".into(),
                gateway: "https://alpha4-2.starknet.io/gateway".into(),
                feeder_gateway: None,
                chain_id: "393402129659245999442226".into(),
            },
        ]
    }

    pub fn abis_dir(&self) -> String {
        [&self.artifacts_dir, "/abis/"].concat()
    }

    fn figment() -> Figment {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(ScarbProvider::new("Scarb.toml"))
            .merge(Env::prefixed("NILE_RS_"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_networks() {
        let network = Config::get_network("localhost").unwrap();
        assert_eq!(network.name, "localhost");

        let network = Config::get_network("mainnet").unwrap();
        assert_eq!(network.name, "mainnet");

        let network = Config::get_network("goerli").unwrap();
        assert_eq!(network.name, "goerli");

        let network = Config::get_network("goerli2").unwrap();
        assert_eq!(network.name, "goerli2");

        let error = Config::get_network("invalid").unwrap_err();
        assert_eq!(format!("{}", error), format!("Network not found!",));
    }

    #[test]
    fn toml_provider() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                "Scarb.toml",
                r#"
            [tool.nile_rs]
            contracts_dir = "other_contracts/"
            artifacts_dir = "other_artifacts/"
            networks = [
                { name = "local1", gateway = "prov1", chain_id = "1" },
                { name = "local2", gateway = "prov2", chain_id = "2" },
            ]
            "#,
            )?;
            let config: Config = Config::get()?;

            assert_eq!(config.artifacts_dir, "other_artifacts/");
            assert_eq!(config.networks.len(), 2);
            Ok(())
        });
    }

    #[test]
    fn env_provider() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("NILE_RS_CONTRACTS_DIR", "other_contracts/");
            jail.set_env("NILE_RS_ARTIFACTS_DIR", "other_artifacts/");

            let config: Config = Config::get()?;

            assert_eq!(config.contracts_dir, "other_contracts/");
            assert_eq!(config.artifacts_dir, "other_artifacts/");
            Ok(())
        });
    }

    #[test]
    fn combined_providers() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                "Scarb.toml",
                r#"
            [tool.nile_rs]
            contracts_dir = "other_contracts/"
            "#,
            )?;
            jail.set_env("NILE_RS_ARTIFACTS_DIR", "other_artifacts/");

            let config: Config = Config::get()?;

            assert_eq!(config.contracts_dir, "other_contracts/");
            assert_eq!(config.artifacts_dir, "other_artifacts/");
            Ok(())
        });
    }

    #[test]
    fn providers_precedence() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("NILE_RS_CONTRACTS_DIR", "contracts_env/");
            jail.create_file(
                "Nile.toml",
                r#"
            contracts_dir = "contracts_toml/"
            "#,
            )?;

            let config: Config = Config::get()?;

            assert_eq!(config.contracts_dir, "contracts_env/");
            Ok(())
        });
    }

    #[test]
    fn abis_dir() {
        figment::Jail::expect_with(|jail| {
            jail.set_env("NILE_RS_ARTIFACTS_DIR", "artifacts_env");

            let config: Config = Config::get()?;
            let abis_dir = config.abis_dir();

            assert_eq!(abis_dir, "artifacts_env/abis/");
            Ok(())
        });
    }

    #[test]
    fn get_network() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                "Scarb.toml",
                r#"
            [tool.nile_rs]
            networks = [
                { name = "local1", gateway = "prov1", chain_id = "1" },
                { name = "local2", gateway = "prov2", chain_id = "2" },
            ]
            "#,
            )?;
            let network = Config::get_network("local2")?;
            assert_eq!(network.name, "local2");

            Ok(())
        });
    }
}
