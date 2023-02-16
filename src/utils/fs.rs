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
        let text = std::fs::read_to_string(&sierra_file).unwrap();

        // Parse the string into a dynamically-typed JSON structure.
        serde_json::from_str::<Value>(&text).unwrap()
    };

    sierra_json["abi"].clone()
}
