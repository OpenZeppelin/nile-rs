use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let script = env::var_os("NILE_RS_TARGET_SCRIPT")
        .unwrap()
        .into_string()
        .unwrap();
    let dest_path = Path::new("./src/main.rs");
    let contents = fs::read_to_string(format!("../{}.rs", script)).expect("Script not found.");
    let with_disclosure = [
        "// Auto-generated file. Don't edit directly.\n\n",
        &contents,
    ]
    .concat();

    fs::write(
        &dest_path,
        with_disclosure
            + r#"
fn main() {
    run();
}
"#,
    )
    .unwrap();
}
