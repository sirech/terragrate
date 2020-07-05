#[macro_use]
extern crate clap;

use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;

use terragrate::migration::Migration;
use terragrate::state::State;

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

fn read_state(file: &str) -> Result<State> {
    match file {
        "-" => State::from_stdin(),
        _ => State::from_file(file),
    }
}

fn main() -> Result<()> {
    let cfg = config();
    let matches = clap_app!(terragrate =>
                            (version: &*cfg.package.version)
                            (author: &*cfg.package.authors.join(","))
                            (about: &*cfg.package.description)
                            (@setting ArgRequiredElseHelp)
                            (@setting ColoredHelp)
                            (@arg STATE: -s --state <STATE_FILE> +takes_value "State file to migrate")
                            (@arg MIGRATION: -m --migration <MIGRATION_FILE> +takes_value "Migration file to use for the migration")
                            (after_help: "When STATE_FILE is '-', read standard input.")
    )
    .get_matches();

    let state = read_state(matches.value_of("STATE").expect("unreachable"))?;
    let migration = Migration::from_file(matches.value_of("MIGRATION").expect("unreachable"))?;

    println!("Migration result: {:?}", migration.apply(&state));
    Ok(())
}
