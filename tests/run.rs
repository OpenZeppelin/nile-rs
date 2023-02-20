use nile_test_utils::{expected_stdout, snapbox::get_snapbox};

#[test]
fn test_run() {
    let assert = get_snapbox().arg("run").arg("-h").assert().success();

    assert.stdout_eq(expected_stdout("run -h"));
}
