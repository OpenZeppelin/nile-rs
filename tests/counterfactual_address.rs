use nile_test_utils::{expected_stdout, snapbox::get_snapbox};

#[test]
fn test_counterfactual_address() {
    let assert = get_snapbox()
        .arg("counterfactual-address")
        .arg("ACCOUNT_1_PK")
        .env("ACCOUNT_1_PK", "1")
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("counterfactual-address"));
}
