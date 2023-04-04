mod common;

use assert_fs::prelude::*;
use httpmock::prelude::*;
use nile_rs::core::Deployments;
use std::env;

use nile_test_utils::{clean_env, expected_stdout, mock_network, snapbox::get_snapbox};

#[test]
fn test_legacy_deploy() {
    let private_key_env = "ACCOUNT_1_PK";
    let network = "localhost";

    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["artifacts/ERC20.json"])
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
        .arg("legacy-deploy")
        .arg("ERC20")
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

    assert.stdout_eq(expected_stdout("legacy_deploy"));

    // Clean env after finishing using the mocked network
    clean_env()
}

#[test]
fn test_fee_estimation() {
    let network = "localhost";

    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["artifacts/ERC20.json"])
        .unwrap();

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    // Mock endpoints
    common::mock_predeployed_accounts_endpoint(&server);
    common::mock_get_nonce_endpoint(&server);
    common::mock_estimate_fee_endpoint(&server);

    let assert = get_snapbox()
        .arg("legacy-deploy")
        .arg("ERC20")
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
