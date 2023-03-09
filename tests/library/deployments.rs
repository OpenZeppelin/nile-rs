use assert_fs::prelude::*;
use std::env;

use nile_rs::core::Deployments;

#[test]
fn test_account_not_found() {
    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["deployments/0.accounts.json"])
        .unwrap();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    let error = Deployments::load_account("ACCOUNT", "0").unwrap_err();
    assert_eq!(
        format!("{}", error),
        "Account not found! If the account is deployed already, \
        try registering it in `deployments/0.accounts.json`"
    );
    assert!(env::set_current_dir(cwd).is_ok());
}

#[test]
fn test_contract_not_found_alias() {
    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["deployments/0.contracts.json"])
        .unwrap();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    let error = Deployments::load_contract_from_alias("0x1", "0").unwrap_err();
    assert_eq!(
        format!("{}", error),
        "Contract not found! If the contract is deployed already, \
        try registering it in `deployments/0.contracts.json`"
    );
    assert!(env::set_current_dir(cwd).is_ok());
}

#[test]
fn test_contract_not_found_address() {
    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["deployments/0.contracts.json"])
        .unwrap();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    let error = Deployments::load_contract_from_address("0x1", "0").unwrap_err();
    assert_eq!(
        format!("{}", error),
        "Contract not found! If the contract is deployed already, \
        try registering it in `deployments/0.contracts.json`"
    );
    assert!(env::set_current_dir(cwd).is_ok());
}

#[test]
fn test_save_account_to_empty_file() {
    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["deployments/empty.accounts.json"])
        .unwrap();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    let error = Deployments::save_account("ACCOUNT", "0x1", "0x2", "empty").unwrap_err();
    assert_eq!(
        format!("{}", error),
        "Failed to load the existing accounts from `deployments/empty.accounts.json`.\
        \nTry removing the file if it is empty."
    );
    assert!(env::set_current_dir(cwd).is_ok());
}

#[test]
fn test_save_contract_to_empty_file() {
    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["deployments/empty.contracts.json"])
        .unwrap();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    let error = Deployments::save_contract(None, "0x1", "empty").unwrap_err();
    assert_eq!(
        format!("{}", error),
        "Failed to load the existing contracts from `deployments/empty.contracts.json`.\
        \nTry removing the file if it is empty."
    );
    assert!(env::set_current_dir(cwd).is_ok());
}
