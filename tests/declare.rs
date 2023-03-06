use assert_fs::prelude::*;
use httpmock::prelude::*;
use nile_rs::core::accounts::db::DB;
use serde_json::json;
use serial_test::serial;
use std::env;

use nile_test_utils::{expected_stdout, mock_network, snapbox::get_snapbox};

#[test]
#[serial]
fn test_declare() {
    let private_key_env = "ACCOUNT_1_PK";
    let network = "localhost";

    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["artifacts/contract.json"])
        .unwrap();

    let cwd = env::current_dir().unwrap();
    assert!(env::set_current_dir(&temp).is_ok());
    DB::save_account(private_key_env, "0x1", "0x2", network).unwrap();
    assert!(env::set_current_dir(cwd).is_ok());

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    server.mock(|when, then| {
        when.path("/feeder_gateway/get_nonce");
        then.status(200).body("\"0x0\"");
    });
    server.mock(|when, then| {
        when.path("/gateway/add_transaction");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
              "code": "TRANSACTION_RECEIVED",
              "transaction_hash": "0x376fc5328badc4eff64d0332044a9b455f264e5014d46af5880fe4df43f9f1e",
              "class_hash": "0x508fc648f7dc864be1242384cc819f0d23bfeea97b5216923ab769e103c9775"}));
    });

    let assert = get_snapbox()
        .arg("declare")
        .arg("contract")
        .arg("-p")
        .arg(private_key_env)
        .arg("--network")
        .arg(network)
        .arg("--max-fee")
        .arg("1")
        .env("ACCOUNT_1_PK", "1")
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("declare"));
}

#[test]
#[serial]
fn test_estimate_fee() {
    let network = "localhost";

    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["artifacts/contract.json"])
        .unwrap();

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    server.mock(|when, then| {
        when.path("/predeployed_accounts");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!([
            { "private_key": "0x1", "address": "0x2" },
            { "private_key": "0x3", "address": "0x4" }]));
    });
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
        .arg("declare")
        .arg("contract")
        .arg("-d")
        .arg("0")
        .arg("--network")
        .arg("localhost")
        .arg("--estimate-fee")
        .env("ACCOUNT_1_PK", "1")
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("estimate_fee"));
}
