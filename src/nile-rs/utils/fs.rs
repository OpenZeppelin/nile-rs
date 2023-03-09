use serde_json::Value;
use walkdir::WalkDir;

/// Return all contracts found under a given directory
pub fn get_all_contracts(directory: &str) -> Vec<String> {
    let mut all_contracts = Vec::<String>::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let f_path = entry.path().to_string_lossy();

        if f_path.ends_with(".cairo") {
            all_contracts.push(f_path.to_string());
        }
    }
    all_contracts
}

/// Extract the ABI from a Sierra artifact
pub fn get_abi_from_sierra(sierra_file: &str) -> Value {
    let sierra_json = {
        // Load the first file into a string.
        let text = std::fs::read_to_string(sierra_file).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    sierra_json["abi"].clone()
}

#[cfg(test)]
mod tests {
    use super::{get_abi_from_sierra, get_all_contracts};

    #[test]
    fn get_all_contracts_output() {
        let dir = "./tests/fixtures/contracts/";
        let contracts = get_all_contracts(dir);

        assert_eq!(contracts.len(), 2);
        assert!(contracts[0].ends_with("/hello_starknet.cairo"));
        assert!(contracts[1].ends_with("/erc20.cairo"));
    }

    #[test]
    fn get_abi_from_sierra_output() {
        let abi = get_abi_from_sierra("./tests/fixtures/artifacts/hello_starknet.sierra");

        assert_eq!(abi[0]["name"], "increase_balance");
    }
}
