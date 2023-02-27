mod base_project;

use super::CliCommand;
use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
pub struct Init {}

const BASE_PROJECT_STRUCTURE: [(&str, &str, &str); 7] = [
    ("./", "Cargo.toml", base_project::CARGO_TOML),
    ("./", "Nile.toml", base_project::NILE_TOML),
    ("./", ".gitignore", base_project::GITIGNORE),
    (
        "./contracts/",
        "hello_starknet.cairo",
        base_project::HELLO_STARKNET_CAIRO,
    ),
    ("./scripts/module/", "build.rs", base_project::BUILD_RS),
    (
        "./scripts/module/",
        "Cargo.toml",
        base_project::SCRIPTS_CARGO_TOML,
    ),
    ("./scripts/module/src/", "main.rs", base_project::MAIN_RS),
];

impl CliCommand for Init {
    type Output = ();

    /// Generate base project files
    fn run(self) -> Result<Self::Output> {
        let path = std::env::current_dir().unwrap();
        if path.join("Cargo.toml").exists() {
            anyhow::bail!("`nile-rs init` cannot be run on existing Cargo packages")
        }

        for file in BASE_PROJECT_STRUCTURE {
            copy_file(file.0, file.1, file.2)?
        }

        println!("🗄 Creating project directory tree");
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
fn test_base_project_len() {
    assert_eq!(BASE_PROJECT_STRUCTURE.len(), 7);
}
