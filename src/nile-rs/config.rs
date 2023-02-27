use figment::{
    providers::{Env, Format, Serialized, Toml},
    Error, Figment,
};
use serde::{Deserialize, Serialize};

mod default;

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// Directory containing the Cairo contracts
    pub contracts_dir: String,
    /// Directory where artifacts are stored
    pub artifacts_dir: String,
}

impl Config {
    pub fn get() -> Result<Self, Error> {
        Config::figment().select("nile").extract::<Self>()
    }

    pub fn abis_dir(&self) -> String {
        [&self.artifacts_dir, "/abis/"].concat()
    }

    fn figment() -> Figment {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file("Nile.toml").nested())
            .merge(Env::prefixed("NILE_RS_"))
    }
}

/// Configuration default values
impl Default for Config {
    fn default() -> Config {
        Config {
            contracts_dir: default::DEFAULT_CONTRACTS_DIRECTORY.into(),
            artifacts_dir: default::DEFAULT_BUILD_DIRECTORY.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toml_provider() {
        figment::Jail::expect_with(|jail| {
            jail.create_file(
                "Nile.toml",
                r#"
            [nile]
            contracts_dir = "other_contracts/"
            artifacts_dir = "other_artifacts/"
          "#,
            )?;
            let config: Config = Config::get()?;

            assert_eq!(config.contracts_dir, "other_contracts/");
            assert_eq!(config.artifacts_dir, "other_artifacts/");
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
                "Nile.toml",
                r#"
            [nile]
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
            [nile]
            contracts_dir = "contracts_toml/"
          "#,
            )?;

            let config: Config = Config::get()?;

            assert_eq!(config.contracts_dir, "contracts_toml/");
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
}
