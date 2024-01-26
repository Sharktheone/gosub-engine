use std::env;
use syn::Path;

pub fn get_crate() -> Path {
    let mut name = env::var("CARGO_PKG_NAME").unwrap();
    if name == "gosub-engine" {
        name = "crate".to_string();
    }

    let name = name.replace('-', "_");

    syn::parse_str::<Path>(&name).unwrap()
}
