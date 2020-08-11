#[macro_use]
extern crate clap;

use anyhow::Result;

use terragrate::command::Commands;
use terragrate::migration::Migration;
use terragrate::state::State;

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

fn print_commands(result: &Commands) {
    for line in result.elements.iter() {
        println!("{}", line);
    }
}

fn main() -> Result<()> {
    let matches = clap_app!(terragrate =>
                            (version: crate_version!())
                            (author: env!("CARGO_PKG_AUTHORS"))
                            (about: env!("CARGO_PKG_DESCRIPTION"))
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
                            (@subcommand tf =>
                             (about: "Prints the terraform commands to execute the given migration")
                            )
                            (after_help: "When STATE_FILE is '-', read standard input.")
    )
    .get_matches();

    let state = read_state(matches.value_of("STATE").expect("unreachable"))?;
    let migration = Migration::from_file(matches.value_of("MIGRATION").expect("unreachable"))?;
    let (result, commands) = migration.apply(&state);

    if matches.subcommand_matches("initial_state").is_some() {
        print_state(&state);
    }

    if matches.subcommand_matches("end_state").is_some() {
        print_state(&result);
    }

    if matches.subcommand_matches("diff").is_some() {
        print_diff(&state, &result);
    }

    if matches.subcommand_matches("tf").is_some() {
        print_commands(&commands);
    }

    Ok(())
}
