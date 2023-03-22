mod base_project;

use super::CliCommand;
use anyhow::{Context, Ok, Result};
use async_trait::async_trait;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
pub struct Init {}

const BASE_PROJECT_STRUCTURE: [(&str, &str, &str); 9] = [
    ("./", "Cargo.toml", base_project::CARGO_TOML),
    ("./", "Scarb.toml", base_project::SCARB_TOML),
    ("./", ".gitignore", base_project::GITIGNORE),
    (
        "./src/",
        "hello_starknet.cairo",
        base_project::HELLO_STARKNET_CAIRO,
    ),
    ("./src/", "lib.cairo", base_project::LIB_CAIRO),
    ("./scripts/.module/", "build.rs", base_project::BUILD_RS),
    (
        "./scripts/.module/",
        "Cargo.toml",
        base_project::SCRIPTS_CARGO_TOML,
    ),
    ("./scripts/.module/src/", "main.rs", base_project::MAIN_RS),
    ("./scripts/", "example.rs", base_project::EXAMPLE_SCRIPT),
];

#[async_trait]
impl CliCommand for Init {
    type Output = ();

    /// Generate base project files
    async fn run(&self) -> Result<Self::Output> {
        let path = std::env::current_dir().unwrap();
        if path.join("Cargo.toml").exists() {
            anyhow::bail!("`nile-rs init` cannot be run on existing Cargo packages")
        }

        for file in BASE_PROJECT_STRUCTURE {
            copy_file(file.0, file.1, file.2)?
        }

        println!("🗄  Creating project directory tree");
        println!("⛵️ Nile project ready!");

        Ok(())
    }
}

fn copy_file(to_dir: &str, file: &str, contents: &str) -> Result<()> {
    let out_path = [to_dir, file].concat();

    if to_dir != "./" {
        fs::create_dir_all(to_dir)?;
    }
    fs::write(&out_path, contents)
        .with_context(|| format!("Failed to write contents to {}", out_path))?;

    Ok(())
}

#[test]
fn base_project_len() {
    assert_eq!(BASE_PROJECT_STRUCTURE.len(), 9);
}
