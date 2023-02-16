use assert_cmd::Command;
use utils::expected_stdout;

mod utils;

#[test]
fn test_1() {
    let mut cmd = Command::cargo_bin("nile-rs").unwrap();
    let assert = cmd
        .arg("compile")
        .arg("tests/fixtures/hello_starknet.cairo")
        .assert();

    assert.stdout(expected_stdout("compile"));
}
