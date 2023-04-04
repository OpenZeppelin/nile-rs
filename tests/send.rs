mod common;

use httpmock::prelude::*;
use nile_rs::core::Deployments;
use std::env;

use nile_test_utils::{clean_env, expected_stdout, mock_network, snapbox::get_snapbox};

const CONTRACT_ADDRESS: &str = "0x07cfadda3ed391f56ba9a556457bb102c0965fef2a254e750a7ce2b85458a7b0";

#[test]
fn test_send() {
    let private_key_env = "ACCOUNT_1_PK";
    let network = "localhost";

    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    Deployments::save_account(private_key_env, "0x1", "0x2", network).unwrap();
    Deployments::save_contract(None, CONTRACT_ADDRESS, network).unwrap();
    assert!(env::set_current_dir(cwd).is_ok());

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    // Mock endpoints
    common::mock_get_status_endpoint(&server);
    common::mock_get_nonce_endpoint(&server);
    common::mock_add_transaction_endpoint(&server);

    let assert = get_snapbox()
        .arg("send")
        .arg("-p")
        .arg(private_key_env)
        .arg("--address")
        .arg(CONTRACT_ADDRESS)
        .arg("--network")
        .arg(network)
        .arg("--max-fee")
        .arg("1")
        .arg("--track")
        .arg("transfer")
        .arg("1")
        .arg("0")
        .arg("1")
        .env("ACCOUNT_1_PK", "1")
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("send"));

    // Clean env after finishing using the mocked network
    clean_env()
}

#[test]
fn test_fee_estimation() {
    let network = "localhost";
    let temp = assert_fs::TempDir::new().unwrap();

    // Register the contract locally
    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    Deployments::save_contract(None, CONTRACT_ADDRESS, network).unwrap();
    assert!(env::set_current_dir(cwd).is_ok());

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    // Mock endpoints
    common::mock_predeployed_accounts_endpoint(&server);
    common::mock_get_nonce_endpoint(&server);
    common::mock_estimate_fee_endpoint(&server);

    let assert = get_snapbox()
        .arg("send")
        .arg("-d")
        .arg("0")
        .arg("--address")
        .arg(CONTRACT_ADDRESS)
        .arg("--network")
        .arg(network)
        .arg("transfer")
        .arg("1")
        .arg("0")
        .arg("1")
        .arg("-e")
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("estimate_fee"));

    // Clean env after finishing using the mocked network
    clean_env()
}
