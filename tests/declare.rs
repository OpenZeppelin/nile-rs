use assert_fs::prelude::*;
use httpmock::prelude::*;
use nile_rs::core::accounts::db::DB;
use serde_json::json;
use std::env;

use nile_test_utils::{mock_network, snapbox::get_snapbox};

#[test]
fn test_declare() {
    let private_key_env = "ACCOUNT_1_PK";
    let network = "localhost";

    // Register the account locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["artifacts/contract.json"])
        .unwrap();

    assert!(env::set_current_dir(&temp).is_ok());
    DB::save_account(private_key_env, "0x1", "0x2", network).unwrap();

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

    assert.success();
}
