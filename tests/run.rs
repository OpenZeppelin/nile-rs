use nile_test_utils::{expected_stdout, snapbox::get_snapbox};

#[test]
fn test_run() {
    let pt = assert_fs::TempDir::new().unwrap();
    let assert = get_snapbox()
        .current_dir(&pt)
        .arg("run")
        .arg("declare")
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("run"));
}
