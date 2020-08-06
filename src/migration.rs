use crate::command::{Command, Commands};
use crate::element::Element;
use crate::state::State;

#[allow(unused_imports)]
use crate::transformation::{Transformation, TransformationType};
use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Migration {
    name: String,
    description: String,
    transformations: Vec<Transformation>,
}

impl Migration {
    pub fn from_file(file_name: &str) -> Result<Self> {
        let content = fs::read_to_string(file_name)?;
        let result = serde_json::from_str(&content)?;
        Ok(result)
    }

    pub fn apply(&self, state: &State) -> (State, Commands) {
        let mut new_elements = Vec::new();
        let mut commands = Vec::new();

        for element in state.elements.iter() {
            let (new_element, mut new_commands) = self.apply_element(element);
            new_elements.push(new_element);
            commands.append(&mut new_commands);
        }

        (
            State {
                elements: new_elements,
            },
            Commands {
                elements: commands
                    .into_iter()
                    .filter(|c| *c != Command::NoOp)
                    .collect(),
            },
        )
    }

    fn apply_element(&self, element: &Element) -> (Element, Vec<Command>) {
        let mut acc = element.clone();
        let mut commands = Vec::new();

        for transformation in self.transformations.iter() {
            let transformation_result = transformation.apply(&acc);
            acc = transformation_result.element;
            commands.push(transformation_result.command);
        }

        (acc, commands)
    }
}

#[cfg(test)]
mod migration_tests {
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
        let m = Migration {
            name: "move to module".to_string(),
            description: "Convert network to module".to_string(),
            transformations: vec![Transformation {
                kind: TransformationType::MV,
                matcher: "public_network".to_string(),
                replacement: "module.public_network".to_string(),
            }],
        };

        assert_eq!(
            Migration::from_file("fixtures/migrations/move_to_module.json").unwrap(),
            m
        )
    }

    #[test]
    fn test_apply_migration() {
        let m = Migration::from_file("fixtures/migrations/multi_step.json").unwrap();

        let target = State {
            elements: vec![
                Element::Resource("docker_container.container".to_string()),
                Element::Resource("docker_image.image".to_string()),
                Element::Resource("module.public_network.docker_network.network".to_string()),
            ],
        };

        let expected_commands = Commands {
            elements: vec![Command::MV {
                source: "public_network.docker_network.network".to_string(),
                target: "module.public_network.docker_network.network".to_string(),
            }],
        };

        let (state, commands) = m.apply(&current());
        assert_eq!(target, state);
        assert_eq!(expected_commands, commands);
    }
}
