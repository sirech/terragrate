use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Command {
    MV { source: String, target: String },
    RM(String),
    NoOp,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Command::MV { source, target } => format!("mv {} {}", source, target),
            Command::RM(s) => format!("rm {}", s),
            Command::NoOp => panic!("NoOp cannot be translated to a string"),
        };
        write!(f, "{}", format!("terraform state {}", str))
    }
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

    #[test]
    fn test_move_command_to_string() {
        let command = Command::MV {
            source: "public_network.docker_network.network".to_string(),
            target: "module.public_network.docker_network.network".to_string(),
        };

        assert_eq!("terraform state mv public_network.docker_network.network module.public_network.docker_network.network", command.to_string())
    }

    #[test]
    fn test_rm_command_to_string() {
        let command = Command::RM("public_network.docker_network.network".to_string());

        assert_eq!(
            "terraform state rm public_network.docker_network.network",
            command.to_string()
        )
    }
}
