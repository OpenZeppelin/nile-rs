use nile_test_utils::snapbox::get_snapbox;

#[test]
fn test_setup() {
    let assert = get_snapbox()
        .arg("setup")
        .arg("--network")
        .arg("goerli")
        .arg("--max-fee")
        .arg("1")
        .arg("ACCOUNT_1_PK")
        .env("ACCOUNT_1_PK", "1")
        .assert()
        .success();

    assert.success();
}
