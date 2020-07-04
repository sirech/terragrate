use crate::transformation::Transformation;
use anyhow::Result;
use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize, PartialEq, Debug)]
struct Migration {
    name: String,
    description: String,
    transformations: Vec<Transformation>,
}

impl Migration {
    fn from_file(file_name: &str) -> Result<Self> {
        let content = fs::read_to_string(file_name)?;
        let result = serde_json::from_str(&content)?;
        Ok(result)
    }
}

#[cfg(test)]
mod migration_tests {
    use super::*;

    #[test]
    fn test_from_file_loads_a_migration() {
        let m = Migration {
            name: "move to module".to_string(),
            description: "Convert network to module".to_string(),
            transformations: vec![Transformation {
                matcher: "public_network".to_string(),
                replacement: "module.public_network".to_string(),
            }],
        };

        assert_eq!(
            Migration::from_file("fixtures/migrations/move_to_module.json").unwrap(),
            m
        )
    }
}
