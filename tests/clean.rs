use nile_test_utils::{expected_stdout, snapbox::get_snapbox};

#[test]
fn test_clean() {
    let pt = assert_fs::TempDir::new().unwrap();

    let assert = get_snapbox()
        .arg("clean")
        .current_dir(&pt)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("clean"));
}
