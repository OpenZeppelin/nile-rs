mod common;

use httpmock::prelude::*;
use nile_test_utils::{clean_env, expected_stdout, mock_network, snapbox::get_snapbox};

#[test]
fn test_get_nonce() {
    let address = "0x00e2f6d939b55f5b8622ec8323a58c8a2802c029409793852b44433ddcfee023";
    let network = "localhost";

    // Mock the provider
    let server = MockServer::start();
    mock_network(network, &server.url("/gateway"));

    // Mock endpoints
    common::mock_get_nonce_endpoint(&server);

    let assert = get_snapbox()
        .arg("get-nonce")
        .arg(address)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("get-nonce"));

    // Clean env after finishing using the mocked network
    clean_env()
}
