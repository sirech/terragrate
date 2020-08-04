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

fn print_state(state: &State) {
    for line in state.elements.iter() {
        println!("{}", line)
    }
}

fn print_diff(state: &State, result: &State) {
    let diff = state.diff(&result);
    for line in diff.iter() {
        println!("{}", line);
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
                            (@subcommand initial_state =>
                             (about: "Prints the initial state")
                            )
                            (@subcommand end_state =>
                             (about: "Prints the end state after the migration")
                            )
                            (@subcommand diff =>
                             (about: "Show the difference between the existing state and the end state")
                            )
                            (after_help: "When STATE_FILE is '-', read standard input.")
    )
    .get_matches();

    let state = read_state(matches.value_of("STATE").expect("unreachable"))?;
    let migration = Migration::from_file(matches.value_of("MIGRATION").expect("unreachable"))?;
    let result = migration.apply(&state);

    if matches.subcommand_matches("initial_state").is_some() {
        print_state(&state);
    }

    if matches.subcommand_matches("end_state").is_some() {
        print_state(&result);
    }

    if matches.subcommand_matches("diff").is_some() {
        print_diff(&state, &result);
    }

    Ok(())
}
