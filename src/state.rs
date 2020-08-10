use crate::element::Element;
use crate::format_list::format_list;
use anyhow::Result;
use std::fmt;
use std::fs;
use std::io::{self, Read};

#[derive(PartialEq, Debug)]
pub struct State {
    pub elements: Vec<Element>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{}", format_list(&self.elements)))
    }
}

impl State {
    pub fn from_file(file_name: &str) -> Result<Self> {
        let content = fs::read_to_string(file_name)?;
        Ok(State::new(&content))
    }

    // TODO: cover in test
    pub fn from_stdin() -> Result<Self> {
        let mut buffer = String::new();
        io::stdin().lock().read_to_string(&mut buffer)?;
        Ok(State::new(&buffer))
    }

    fn new(content: &str) -> Self {
        Self {
            elements: content
                .lines()
                .map(|l| Element::Resource(l.to_string()))
                .collect(),
        }
    }

    pub fn diff(&self, other: &State) -> Vec<String> {
        self.elements
            .iter()
            .zip(other.elements.iter())
            .map(|(src, dst)| src.diff(&dst))
            .collect()
    }
}

#[cfg(test)]
mod state_tests {
    use super::*;

    fn current() -> State {
        State {
            elements: vec![
                Element::Resource("docker_container.container".to_string()),
                Element::Resource("docker_image.image".to_string()),
                Element::Resource("public_network.docker_network.network".to_string()),
            ],
        }
    }

    #[test]
    fn test_from_file_loads_a_migration() {
        assert_eq!(
            State::from_file("fixtures/state/current").unwrap(),
            current()
        )
    }

    #[test]
    fn test_diff_compares_state_line_by_line() {
        let lines: Vec<String> = current().elements.iter().map(|l| l.to_string()).collect();
        assert_eq!(
            State::from_file("fixtures/state/current")
                .unwrap()
                .diff(&current()),
            lines
        )
    }
}
