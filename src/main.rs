#[macro_use]
extern crate clap;

use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Config {
    package: Package,
}

#[derive(Deserialize, Debug)]
struct Package {
    authors: Vec<String>,
    version: String,
    description: String,
}

fn config() -> Config {
    let content = fs::read_to_string("Cargo.toml").unwrap();
    toml::from_str(&content).unwrap()
}

fn main() {
    let cfg = config();
    let matches = clap_app!(terragrate =>
                            (version: &*cfg.package.version)
                            (author: &*cfg.package.authors.join(","))
                            (about: &*cfg.package.description)
    )
    .get_matches();
}
