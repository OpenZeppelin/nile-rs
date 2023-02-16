use assert_cmd::Command;
mod utils;

use utils::expected_stdout;

#[test]
fn test_1() {
    let mut cmd = Command::cargo_bin("nile-rs").unwrap();
    let assert = cmd
        .arg("compile")
        .arg("tests/fixtures/hello_starknet.cairo")
        .assert();

    assert.stdout(expected_stdout("compile"));
}
