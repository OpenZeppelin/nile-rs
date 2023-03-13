use httpmock::prelude::*;
use serde_json::json;

use nile_test_utils::{clean_env, expected_stdout, mock_network, snapbox::get_snapbox};

const CONTRACT_ADDRESS: &str = "0x07cfadda3ed391f56ba9a556457bb102c0965fef2a254e750a7ce2b85458a7b0";

#[test]
fn test_raw_call() {
    let network = "localhost";

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    server.mock(|when, then| {
        when.path("/feeder_gateway/call_contract");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"result": [
              "0x0",
              "0x0"
            ]}));
    });

    let assert = get_snapbox()
        .arg("raw-call")
        .arg(CONTRACT_ADDRESS)
        .arg("--network")
        .arg(network)
        .arg("balanceOf")
        .arg("1")
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("raw-call"));

    // Clean env after finishing using the mocked network
    clean_env()
}
