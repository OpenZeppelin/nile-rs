pub mod snapbox;

use std::fs;

const STDOUT_DIR: &str = "tests/fixtures/stdout/";

pub fn expected_stdout(cmd: &str) -> String {
    let file_path = [STDOUT_DIR, cmd, ".stdout"].concat();
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
