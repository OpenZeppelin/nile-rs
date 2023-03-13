pub const CARGO_TOML: &str = r#"[workspace]
members = ["scripts/module"]
"#;

pub const NILE_TOML: &str = r#"[nile]
contracts_dir = "contracts/"
artifacts_dir = "artifacts/"
"#;

pub const HELLO_STARKNET_CAIRO: &str = r##"#[contract]
mod HelloStarknet {
    struct Storage {
        balance: felt,
    }

    // Increases the balance by the given amount.
    #[external]
    fn increase_balance(amount: felt) {
        balance::write(balance::read() + amount);
    }

    // Returns the current balance.
    #[view]
    fn get_balance() -> felt {
        balance::read()
    }
}
"##;

pub const GITIGNORE: &str = r#"/target
/artifacts
"#;

pub const BUILD_RS: &str = r##"use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let script = env::var("NILE_RS_TARGET_SCRIPT_NAME").unwrap();
    let network = env::var("NILE_RS_TARGET_SCRIPT_NETWORK").unwrap();

    let dest_path = Path::new("./src/main.rs");
    let contents = fs::read_to_string(format!("../{}.rs", script)).expect("Script not found.");
    let with_disclosure = [
        "// Auto-generated file. Don't edit directly.\n\n",
        &contents,
    ]
    .concat();

    fs::write(
        dest_path,
        with_disclosure
            + &r#"
extern crate nile_rs;
use nile_rs::nre::NileRuntimeEnvironment;

#[tokio::main]
async fn main() {
    let nre = NileRuntimeEnvironment::new("<network>").unwrap();
    run(nre).await;
}
"#
            .replace("<network>", &network),
    )
    .unwrap();
}
"##;

pub const MAIN_RS: &str = r#"// Auto-generated file. Don't edit directly.
fn main() {}
"#;

pub const SCRIPTS_CARGO_TOML: &str = r#"[package]
edition = "2021"
name = "nile-rs-scripts-module"
version = "0.1.0"

[dependencies]
tokio = { version = "1"}
nile-rs = { path = "../../../" }
"#;
