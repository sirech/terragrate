use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub package: Package,
}

#[derive(Deserialize, Debug)]
pub struct Package {
    pub authors: Vec<String>,
    pub version: String,
    pub description: String,
}

pub fn config() -> Config {
    let content = fs::read_to_string("Cargo.toml").unwrap();
    toml::from_str(&content).unwrap()
}
