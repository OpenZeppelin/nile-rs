use nile_test_utils::{expected_stdout, snapbox::get_snapbox};

#[test]
fn test_compile() {
    let assert = get_snapbox()
        .arg("compile")
        .arg("tests/fixtures/hello_starknet.cairo")
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("compile"));
}
