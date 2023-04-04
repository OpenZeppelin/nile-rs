mod common;

use httpmock::prelude::*;
use std::env;

use nile_rs::common::get_registered_accounts;
use nile_test_utils::{clean_env, expected_stdout, mock_network, snapbox::get_snapbox};

#[test]
fn test_setup_with_goerli() {
    let network = "goerli";
    let temp = assert_fs::TempDir::new().unwrap();

    let assert = get_snapbox()
        .arg("setup")
        .arg("--network")
        .arg(network)
        .arg("--max-fee")
        .arg("1")
        .arg("--track")
        .arg("ACCOUNT_1_PK")
        .env("ACCOUNT_1_PK", "1")
        .current_dir(&temp)
        .assert()
        .success();

    assert.success();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    env::set_var("ACCOUNT_1_PK", "1");
    let accounts = get_registered_accounts(network).unwrap();
    env::remove_var("ACCOUNT_1_PK");
    assert!(env::set_current_dir(cwd).is_ok());

    assert_eq!(accounts.len(), 1);
}

#[test]
fn test_estimate_fee() {
    let network = "localhost";
    let temp = assert_fs::TempDir::new().unwrap();

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    // Mock endpoints
    common::mock_get_nonce_endpoint(&server);
    common::mock_estimate_fee_endpoint(&server);

    let assert = get_snapbox()
        .arg("setup")
        .arg("ACCOUNT_1_PK")
        .arg("--network")
        .arg(network)
        .arg("--estimate-fee")
        .env("ACCOUNT_1_PK", "1")
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("estimate_fee"));

    // Clean env after finishing using the mocked network
    clean_env()
}
