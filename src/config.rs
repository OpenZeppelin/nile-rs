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
        Config::figment().extract::<Self>()
    }

    pub fn abis_dir(&self) -> String {
        [&self.artifacts_dir, "/abis/"].concat()
    }

    fn figment() -> Figment {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file("nile-rs.toml"))
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
