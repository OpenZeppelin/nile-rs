use crate::config::Config;

/// Directory containing the smart contracts by default
pub const DEFAULT_CONTRACTS_DIRECTORY: &str = "contracts/";
/// Directory where artifacts will be stored
pub const DEFAULT_BUILD_DIRECTORY: &str = "artifacts/";
/// Directory where deployments will be stored
pub const DEFAULT_DEPLOYMENTS_DIRECTORY: &str = "deployments/";

/// Configuration default values
impl Default for Config {
    fn default() -> Config {
        Config {
            contracts_dir: DEFAULT_CONTRACTS_DIRECTORY.into(),
            artifacts_dir: DEFAULT_BUILD_DIRECTORY.into(),
            deployments_dir: DEFAULT_DEPLOYMENTS_DIRECTORY.into(),
            networks: vec![],
        }
    }
}
