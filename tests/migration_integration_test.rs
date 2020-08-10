extern crate terragrate;

use std::fs;

use terragrate::migration::Migration;
use terragrate::state::State;

pub fn load_file(file_name: &str) -> String {
    let content = fs::read_to_string(file_name).unwrap();
    content
}

#[test]
fn test_executes_migration_correct_commands() {
    let migration = Migration::from_file("fixtures/migrations/complex.json").unwrap();
    let state = State::from_file("fixtures/state/full").unwrap();

    let expected_commands = load_file("fixtures/output/complex_full_commands");
    let (_, commands) = migration.apply(&state);

    assert_eq!(expected_commands, commands.to_string());
}

#[test]
fn test_executes_migration_correct_state() {
    let migration = Migration::from_file("fixtures/migrations/complex.json").unwrap();
    let state = State::from_file("fixtures/state/full").unwrap();

    let expected_state = load_file("fixtures/output/complex_full_state");
    let (result, _) = migration.apply(&state);

    assert_eq!(expected_state, result.to_string());
}
