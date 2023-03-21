pub mod snapbox;

use std::fs;

const STDOUT_DIR: &str = "tests/fixtures/stdout/";

pub fn expected_stdout(cmd: &str) -> String {
    let without_whitespaces: String = cmd.split_whitespace().collect();
    let file_name = without_whitespaces.replace("-", "_");
    let file_path = [STDOUT_DIR, &file_name, ".stdout"].concat();
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}

pub fn mock_network(network: &str, gateway: &str) {
    let networks = [
        "[{ name = \"",
        network,
        "\", chain_id = \"1\", gateway = \"",
        gateway,
        "\"}]",
    ]
    .concat();

    std::env::set_var("NILE_RS_NETWORKS", networks);
}

pub fn clean_env() {
    std::env::remove_var("NILE_RS_NETWORKS");
}
