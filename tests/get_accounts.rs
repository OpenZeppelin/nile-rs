mod common;

use assert_fs::prelude::*;
use nile_test_utils::{expected_stdout, snapbox::get_snapbox};

#[test]
fn test_get_accounts() {
    let network = "localhost";

    // Register the accounts locally
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from(
        "./tests/fixtures",
        &["deployments/localhost.accounts.json", ".env"],
    )
    .unwrap();

    let assert = get_snapbox()
        .arg("get-accounts")
        .arg("--network")
        .arg(network)
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("get-accounts"));
}
