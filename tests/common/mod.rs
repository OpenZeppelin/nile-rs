#![allow(dead_code)]
use httpmock::prelude::*;
use serde_json::json;

pub fn mock_get_status_endpoint(server: &MockServer) {
    server.mock(|when, then| {
        when.path("/feeder_gateway/get_transaction_status");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!(
          { "tx_status": "ACCEPTED_ON_L2" }));
    });
}

pub fn mock_get_nonce_endpoint(server: &MockServer) {
    server.mock(|when, then| {
        when.path("/feeder_gateway/get_nonce");
        then.status(200).body("\"0x0\"");
    });
}

pub fn mock_add_transaction_endpoint(server: &MockServer) {
    server.mock(|when, then| {
        when.path("/gateway/add_transaction");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
              "code": "TRANSACTION_RECEIVED",
              "transaction_hash": "0x376fc5328badc4eff64d0332044a9b455f264e5014d46af5880fe4df43f9f1e",
              "class_hash": "0x508fc648f7dc864be1242384cc819f0d23bfeea97b5216923ab769e103c9775"}));
    });
}

pub fn mock_predeployed_accounts_endpoint(server: &MockServer) {
    server.mock(|when, then| {
        when.path("/predeployed_accounts");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!([
        { "private_key": "0x1", "address": "0x2" },
        { "private_key": "0x3", "address": "0x4" }]));
    });
}

pub fn mock_estimate_fee_endpoint(server: &MockServer) {
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
}
