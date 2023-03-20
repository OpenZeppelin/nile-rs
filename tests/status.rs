use nile_test_utils::{expected_stdout, snapbox::get_snapbox};

#[test]
fn test_status_goerli() {
    let assert = get_snapbox()
        .arg("status")
        .arg("--network")
        .arg("goerli")
        .arg("0x07a4484950cabaccfa1751f30a2587665147bddbffeb209f90bb746321425ac1")
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("status"));
}
