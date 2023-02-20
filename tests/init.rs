use nile_test_utils::{expected_stdout, snapbox};

#[test]
fn test_init() {
    let pt = assert_fs::TempDir::new().unwrap();

    let assert = snapbox::snapbox()
        .arg("init")
        .current_dir(&pt)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("init"));
}
