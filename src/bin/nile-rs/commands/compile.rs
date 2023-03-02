pub mod cairo_to_sierra;
pub mod sierra_to_casm;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

use super::CliCommand;
use cairo_to_sierra::CompileCairoToSierra;
use nile_rs::config::Config;
use nile_rs::utils::fs::{get_abi_from_sierra, get_all_contracts};
use sierra_to_casm::CompileSierraToCasm;

#[derive(Parser, Debug)]
pub struct Compile {
    #[clap(help = "List of contracts to compile.", value_name = "CONTRACTS")]
    pub contracts: Option<Vec<String>>,

    #[clap(
        help = "Compile all contracts inside the specified directory",
        long,
        short,
        value_name = "DIR"
    )]
    pub directory: Option<String>,
}

#[async_trait]
impl CliCommand for Compile {
    type Output = ();

    async fn run(&self) -> Result<Self::Output> {
        let config = Config::get()?;
        let abis_dir = config.abis_dir();

        let contracts = self.contracts.clone();
        let directory = self.directory.clone();

        let contracts_directory = match directory {
            Some(dir) => dir,
            None => config.contracts_dir,
        };

        let contracts = match contracts {
            Some(c) => c,
            None => Vec::new(),
        };

        // Create the artifacts folders if don't exist
        fs::create_dir_all(&abis_dir)?;

        let mut failures = Vec::new();
        let all_contracts = if contracts.is_empty() {
            println!("🤖 Compiling all Cairo contracts in the {contracts_directory} directory");
            get_all_contracts(&contracts_directory)
        } else {
            contracts.to_vec()
        };

        // Compile the contracts
        for contract_file in all_contracts.iter() {
            let file_name = Path::new(contract_file)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();

            let sierra_file = [&config.artifacts_dir, "/", file_name, ".sierra"].concat();

            let compiler = CompileCairoToSierra {
                path: Option::Some(PathBuf::from(&contract_file)),
                output: Option::Some(PathBuf::from(&sierra_file)),
                replace_ids: false,
            };

            let result = compiler.run().await;
            match result {
                std::result::Result::Ok(_) => {
                    // Extract ABI
                    let abi_json = get_abi_from_sierra(&sierra_file);

                    let abi_file = [&abis_dir, file_name, ".json"].concat();
                    fs::write(abi_file, serde_json::to_string_pretty(&abi_json)?)?;

                    // Compile to Casm
                    let casm_file = [&config.artifacts_dir, "/", file_name, ".casm"].concat();
                    let compiler = CompileSierraToCasm {
                        path: Option::Some(PathBuf::from(&sierra_file)),
                        output: Option::Some(PathBuf::from(&casm_file)),
                    };
                    compiler.run().await?;
                }
                Err(_) => {
                    failures.push(contract_file);
                }
            }
        }

        if failures.is_empty() {
            println!("✅ Done");
        } else {
            println!("\n🛑 Failed to compile the following contracts:");
            for contract in failures {
                println!("   {contract}");
            }
        }

        Ok(())
    }
}
