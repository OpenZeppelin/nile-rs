use httpmock::prelude::*;
use serde_json::json;
use std::env;

use nile_rs::common::get_accounts;
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
        .arg("ACCOUNT_1_PK")
        .env("ACCOUNT_1_PK", "1")
        .current_dir(&temp)
        .assert()
        .success();

    assert.success();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    env::set_var("ACCOUNT_1_PK", "1");
    let accounts = get_accounts(network).unwrap();
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

    server.mock(|when, then| {
        when.path("/feeder_gateway/get_nonce");
        then.status(200).body("\"0x0\"");
    });
    server.mock(|when, then| {
        when.path("/feeder_gateway/estimate_fee");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
              "gas_price": 1000000,
              "gas_usage": 1349,
              "overall_fee": 1349000000,
              "unit": "wei"
            }));
    });

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
