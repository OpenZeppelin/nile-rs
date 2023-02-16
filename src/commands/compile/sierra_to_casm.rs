use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use cairo_lang_starknet::casm_contract_class::CasmContractClass;
use cairo_lang_starknet::contract_class::ContractClass;
use clap::Parser;

use super::Compiler;

#[derive(Parser, Debug)]
pub struct CompileSierraToCasm {
    #[clap(
        help = "The path to the Sierra file.",
        long,
        short,
        value_name = "PATH"
    )]
    pub path: Option<PathBuf>,

    #[clap(
        help = "The path to the output Casm file.",
        long = "out",
        short,
        value_name = "OUTPUT"
    )]
    pub output: Option<PathBuf>,
}

impl Compiler for CompileSierraToCasm {
    type Output = CasmContractClass;

    fn run(self) -> Result<Self::Output> {
        let path = self.path.unwrap();

        let contract_class: ContractClass = serde_json::from_str(
            &fs::read_to_string(&path)
                .with_context(|| format!("Failed to read {}.", &path.display()))?,
        )
        .with_context(|| "deserialization Failed.")?;

        let casm_contract_class = CasmContractClass::from_contract_class(contract_class)
            .with_context(|| "Compilation failed.")?;
        let res = serde_json::to_string_pretty(&casm_contract_class)
            .with_context(|| "Serialization failed.")?;

        match self.output {
            Some(path) => fs::write(path, res).with_context(|| "Failed to write output.")?,
            None => println!("{}", res),
        }
        Ok(casm_contract_class)
    }
}
