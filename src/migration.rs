use crate::element::Element;
use crate::state::State;
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

    pub fn apply(&self, state: &State) -> State {
        let new_elements = state
            .elements
            .iter()
            .map(|e| self.apply_element(e))
            .collect();
        State {
            elements: new_elements,
        }
    }

    fn apply_element(&self, element: &Element) -> Element {
        self.transformations
            .iter()
            .fold(element.clone(), |e, t| t.apply(&e))
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
    fn test_apply_simple_migration() {
        let m = Migration::from_file("fixtures/migrations/move_to_module.json").unwrap();

        let target = State {
            elements: vec![
                Element::Resource("docker_container.container".to_string()),
                Element::Resource("docker_image.image".to_string()),
                Element::Resource("module.public_network.docker_network.network".to_string()),
            ],
        };

        assert_eq!(target, m.apply(&current()));
    }
}
