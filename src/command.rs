#[derive(PartialEq, Debug)]
pub enum Command {
    MV { source: String, target: String },
    RM(String),
    NoOp,
}

#[derive(PartialEq, Debug)]
pub struct Commands {
    pub elements: Vec<Command>,
}

impl Commands {
    pub fn new(elements: Vec<Command>) -> Self {
        Self {
            elements: elements
                .into_iter()
                .filter(|c| *c != Command::NoOp)
                .collect(),
        }
    }
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[test]
    fn test_new_filters_noops() {
        let elements = vec![
            Command::NoOp,
            Command::MV {
                source: "public_network.docker_network.network".to_string(),
                target: "module.public_network.docker_network.network".to_string(),
            },
            Command::NoOp,
        ];

        let commands = Commands::new(elements);
        assert_eq!(1, commands.elements.len());
    }
}
