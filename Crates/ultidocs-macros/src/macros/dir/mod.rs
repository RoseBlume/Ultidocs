use std::path::PathBuf;

mod helpers;
use helpers::{
    parse_input,
    generate_runtime_types,
    generate_dir
};
pub fn perform_dir(input_string: String) -> String {
    let (name, path, ignores) = parse_input(&input_string);

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = PathBuf::from(manifest_dir).join(path);

    let mut code = String::new();

    code.push_str(&generate_runtime_types());

    let tree = generate_dir(&root, &root, &ignores);

    code.push_str(&format!(
        "pub struct {};\nimpl {} {{ pub const ROOT: Dir = {}; }}",
        name, name, tree
    ));

    code
}