use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use cairo_lang_starknet::contract_class::{compile_path, ContractClass};
use clap::Parser;

use super::Compiler;

#[derive(Parser, Debug)]
pub struct CompileCairoToSierra {
    #[clap(
        help = "The path to the Cairo1 contract file.",
        long,
        short,
        value_name = "PATH"
    )]
    pub path: Option<PathBuf>,

    #[clap(
        help = "The path to the output Sierra file.",
        long = "out",
        short,
        value_name = "OUTPUT"
    )]
    pub output: Option<PathBuf>,

    #[clap(help = "Use human readable ids.", long, short)]
    pub replace_ids: bool,
}

impl Compiler for CompileCairoToSierra {
    type Output = ContractClass;

    fn run(self) -> Result<Self::Output> {
        let path = &self.path.unwrap();
        println!("Compiling {}", path.display());

        let contract_class = compile_path(path, self.replace_ids)?;
        let res = serde_json::to_string_pretty(&contract_class)
            .with_context(|| "Serialization failed.")?;
        match self.output {
            Some(path) => fs::write(path, res).with_context(|| "Failed to write output.")?,
            None => println!("{res}"),
        }

        Ok(contract_class)
    }
}
