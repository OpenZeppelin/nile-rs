use assert_fs::{prelude::PathCopy, TempDir};
use nile_test_utils::{expected_stdout, snapbox::get_snapbox};

use std::{fs, path::Path};

// Helper function to create a temp dir with a Scarb.toml file
fn temp_dir_with_scarb() -> TempDir {
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("./tests/fixtures", &["Scarb.toml"]).unwrap();
    temp
}

// Helper function to copy artifacts and contract & account deployments
fn copy_artifacts(temp: &TempDir) {
    let fixtures = Path::new("./tests/fixtures");
    // Copy fixture artifacts
    temp.copy_from(
        fixtures,
        &[
            "deployments/empty.contracts.json",
            "deployments/localhost.accounts.json",
            "artifacts/cairo0_contract.json",
        ],
    )
    .unwrap();
    // Move compiled contract to target/release
    let compilation_path = temp.path().join("target/release");
    fs::create_dir_all(&compilation_path).unwrap();
    fs::copy(
        fixtures.join("artifacts/cairo0_contract.json"),
        compilation_path.join("cairo0_contract.json"),
    )
    .unwrap();
}

#[test]
fn test_clean_none() {
    let temp = temp_dir_with_scarb();

    let assert = get_snapbox()
        .arg("clean")
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("clean_none"));
}

#[test]
fn test_clean() {
    let temp = temp_dir_with_scarb();

    // Copy artifacts and contract and account deployments
    copy_artifacts(&temp);

    let assert = get_snapbox()
        .arg("clean")
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("clean"));
}

#[test]
fn test_clean_all() {
    let temp = temp_dir_with_scarb();

    // Copy artifacts and contract and account deployments
    copy_artifacts(&temp);

    let assert = get_snapbox()
        .args(["clean", "--all"])
        .current_dir(&temp)
        .assert()
        .success();

    assert.stdout_eq(expected_stdout("clean_all"));
}
