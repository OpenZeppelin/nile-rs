pub const CARGO_TOML: &str = r#"[workspace]
members = ["scripts/.module"]
"#;

pub const SCARB_TOML: &str = r#"[package]
name = "nile_project" # the name of the package
version = "0.1.0"    # the current version, obeying semver

[[target.starknet-contract]]

[tool.nile_rs]
artifacts_dir = "./target/release"
contracts_dir = "./src"

# This is the default localhost configuration, the value is added as an example
# and you can safely remove it.
networks = [
    { name = "localhost", gateway = "http://127.0.0.1:5050/gateway", chain_id = "1536727068981429685321" }
]
"#;

pub const HELLO_STARKNET_CAIRO: &str = r##"#[contract]
mod HelloStarknet {
    struct Storage {
        balance: felt252,
    }

    // Increases the balance by the given amount.
    #[external]
    fn increase_balance(amount: felt252) {
        balance::write(balance::read() + amount);
    }

    // Returns the current balance.
    #[view]
    fn get_balance() -> felt252 {
        balance::read()
    }
}
"##;

pub const LIB_CAIRO: &str = r#"mod hello_starknet;
"#;

pub const GITIGNORE: &str = r#"/target
/.env
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
nile-rs = { git = "https://github.com/OpenZeppelin/nile-rs/", tag = "v0.1.0-rc.2"}
"#;

pub const EXAMPLE_SCRIPT: &str = r#"// Requires a devnet node running
async fn run(nre: NileRuntimeEnvironment) {
    println!("Running");

    let accounts = nre.get_predeployed_accounts().await;
    println!("Predeployed accounts: {:?}", accounts);
}
"#;

pub const DOT_ENV: &str = r#"TEST_ACCOUNT = 1234
"#;
