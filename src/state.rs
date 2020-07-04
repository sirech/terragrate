use crate::element::Element;
use anyhow::Result;
use std::fs;

#[derive(PartialEq, Debug)]
pub struct State {
    pub elements: Vec<Element>,
}

impl State {
    fn from_file(file_name: &str) -> Result<State> {
        let content = fs::read_to_string(file_name)?;

        Ok(State {
            elements: content
                .lines()
                .map(|l| Element::Resource(l.to_string()))
                .collect(),
        })
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
}
