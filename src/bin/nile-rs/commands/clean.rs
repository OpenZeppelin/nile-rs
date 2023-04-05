use anyhow::{Context, Result};
use scarb::core::Config;
use scarb::ops;
use std::fs;
use std::path::PathBuf;
use std::thread;

use super::CliCommand;
use async_trait::async_trait;
use clap::Parser;
use nile_rs::config::Config as NileConfig;

#[derive(Parser, Debug)]
pub struct Clean {
    #[clap(
        help = "Scarb manifest path",
        long,
        short,
        default_value = "./Scarb.toml"
    )]
    pub manifest_path: String,
}

#[async_trait]
impl CliCommand for Clean {
    type Output = ();

    // Remove artifacts from workspace
    async fn run(&self) -> Result<Self::Output> {
        // Remove the deployments directory
        let nile_config = NileConfig::get()?;
        let deployments_dir = nile_config.deployments_dir;
        if PathBuf::from(&deployments_dir).exists() {
            fs::remove_dir_all(deployments_dir).unwrap();
            println!("✅ Removed deployments directory");
        } else {
            println!("🟡 No deployments to delete");
        }

        // Remove build artifacts using Scarb
        let src = PathBuf::from(&self.manifest_path);
        let abs_path = fs::canonicalize(src).with_context(|| {
            format!(
                "Unable to clean from the Scarb manifest file: {}",
                &self.manifest_path
            )
        })?;

        let thread = thread::spawn(move || {
            let scarb_config_builder = Config::builder(abs_path.to_str().unwrap());
            let scarb_config = scarb_config_builder.build().unwrap();
            match ops::clean(&scarb_config) {
                Ok(_) => println!("✅ Cleaned Scarb artifacts"),
                Err(_) => println!("🟡 No artifacts to clean"),
            }
        });

        thread.join().expect("Cleaning thread panicked");

        println!("✨ Workspace clean, keep going!");
        Ok(())
    }
}
