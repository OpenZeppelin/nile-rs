mod common;

use assert_fs::prelude::*;
use httpmock::prelude::*;
use nile_rs::core::Deployments;
use std::env;

use nile_test_utils::{clean_env, expected_stdout, mock_network, snapbox::get_snapbox};

#[test]
fn test_declare() {
    let private_key_env = "ACCOUNT_1_PK";
    let network = "localhost";

    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["artifacts/cairo1_contract.json"])
        .unwrap();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    Deployments::save_account(private_key_env, "0x1", "0x2", network).unwrap();
    assert!(env::set_current_dir(cwd).is_ok());

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    // Mock endpoints
    common::mock_get_status_endpoint(&server);
    common::mock_get_nonce_endpoint(&server);
    common::mock_add_transaction_endpoint(&server);

    let assert = get_snapbox()
        .arg("declare")
        .arg("cairo1_contract")
        .arg("-p")
        .arg(private_key_env)
        .arg("--network")
        .arg(network)
        .arg("--max-fee")
        .arg("1")
        .arg("--track")
        .env("ACCOUNT_1_PK", "1")
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("declare"));

    // Clean env after finishing using the mocked network
    clean_env()
}

#[test]
fn test_estimate_fee() {
    let network = "localhost";

    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["artifacts/cairo1_contract.json"])
        .unwrap();

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    // Mock endpoints
    common::mock_get_nonce_endpoint(&server);
    common::mock_predeployed_accounts_endpoint(&server);
    common::mock_estimate_fee_endpoint(&server);

    let assert = get_snapbox()
        .arg("declare")
        .arg("cairo1_contract")
        .arg("-d")
        .arg("0")
        .arg("--network")
        .arg(network)
        .arg("--estimate-fee")
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("estimate_fee"));

    // Clean env after finishing using the mocked network
    clean_env()
}
