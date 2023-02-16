pub mod cairo_to_sierra;
pub mod sierra_to_casm;

use anyhow::{Ok, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::{ABIS_DIRECTORY, BUILD_DIRECTORY, CONTRACTS_DIRECTORY};
use crate::utils::fs::{get_abi_from_sierra, get_all_contracts};
use cairo_to_sierra::CompileCairoToSierra;
use sierra_to_casm::CompileSierraToCasm;

/// Common trait for all compilers
pub trait Compiler {
    type Output;
    fn run(self) -> Result<Self::Output>;
}

#[derive(Parser, Debug)]
pub struct Compile {
    #[clap(help = "The contracts directory.", long, short, value_name = "DIR")]
    pub directory: Option<String>,
}

impl Compiler for Compile {
    type Output = ();

    fn run(self) -> Result<Self::Output> {
        let contracts = Vec::new();
        let directory = self.directory;

        let contracts_directory = match directory {
            Some(dir) => dir,
            None => String::from(CONTRACTS_DIRECTORY),
        };

        // Create the artifacts folders if don't exist
        fs::create_dir_all(ABIS_DIRECTORY)?;

        let mut failures = Vec::new();
        let all_contracts = if contracts.is_empty() {
            println!("🤖 Compiling all Cairo contracts in the {contracts_directory} directory");
            get_all_contracts(&contracts_directory)
        } else {
            contracts
        };

        // Compile the contracts
        for contract_file in all_contracts.iter() {
            let file_name = Path::new(contract_file)
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap();
            let sierra_file = [BUILD_DIRECTORY, file_name, ".sierra"].concat();

            let compiler = CompileCairoToSierra {
                path: Option::Some(PathBuf::from(&contract_file)),
                output: Option::Some(PathBuf::from(&sierra_file)),
                replace_ids: false,
            };

            let result = compiler.run();
            match result {
                std::result::Result::Ok(_) => {
                    // Extract ABI
                    let abi_json = get_abi_from_sierra(&sierra_file);

                    let abi_file = [ABIS_DIRECTORY, file_name, ".json"].concat();
                    fs::write(abi_file, serde_json::to_string_pretty(&abi_json)?)?;

                    // Compile to Casm
                    let casm_file = [BUILD_DIRECTORY, file_name, ".casm"].concat();
                    let compiler = CompileSierraToCasm {
                        path: Option::Some(PathBuf::from(&sierra_file)),
                        output: Option::Some(PathBuf::from(&casm_file)),
                    };
                    compiler.run()?;
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
