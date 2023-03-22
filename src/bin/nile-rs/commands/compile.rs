// use anyhow::Result;

use scarb::core::Config;
use scarb::ops;

use super::CliCommand;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Compile {
    #[clap(
        help = "Scarb manifest path",
        long,
        short,
        default_value = "./Scarb.toml"
    )]
    pub manifest_path: String,
}

#[async_trait]
impl CliCommand for Compile {
    type Output = ();

    // Build the project using Scarb
    async fn run(&self) -> Result<Self::Output> {
        let scarb_config_builder = Config::builder(&self.manifest_path);
        let scarb_config = scarb_config_builder.build()?;
        let ws = ops::read_workspace(scarb_config.manifest_path(), &scarb_config)?;
        ops::compile(&ws)
    }
}
