use crate::command::Command;
use crate::element::Element;
use serde_derive::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub enum TransformationType {
    MV,
    RM,
}

pub struct TransformationResult {
    pub element: Element,
    pub command: Command,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Transformation {
    pub kind: TransformationType,
    pub matcher: String,
    pub replacement: String,
}

impl Transformation {
    pub fn apply(&self, e: &Element) -> TransformationResult {
        match self.kind {
            TransformationType::MV => self.mv(e),
            TransformationType::RM => self.rm(e),
        }
    }

    fn mv(&self, e: &Element) -> TransformationResult {
        let new_element = match e {
            Element::Resource(r) => Element::Resource(r.replace(&self.matcher, &self.replacement)),
            Element::Empty => Element::Empty,
        };

        let command = match new_element {
            Element::Resource(_) if *e == new_element => Command::NoOp,
            Element::Resource(_) => Command::MV {
                source: e.to_string(),
                target: new_element.to_string(),
            },
            Element::Empty => Command::NoOp,
        };

        TransformationResult {
            element: new_element,
            command: command,
        }
    }

    fn rm(&self, e: &Element) -> TransformationResult {
        let (new_element, command) = match e {
            Element::Resource(r) if r.contains(&self.matcher) => {
                (Element::Empty, Command::RM(r.to_string()))
            }
            Element::Resource(_) => (e.clone(), Command::NoOp),
            Element::Empty => (Element::Empty, Command::NoOp),
        };

        TransformationResult {
            element: new_element,
            command: command,
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::*;

    fn element() -> Element {
        Element::Resource("module.public_network.docker_network.network".to_string())
    }

    fn mv() -> Transformation {
        Transformation {
            kind: TransformationType::MV,
            matcher: "public_network".to_string(),
            replacement: "private_network".to_string(),
        }
    }

    fn rm(matcher: &str) -> Transformation {
        Transformation {
            kind: TransformationType::RM,
            matcher: matcher.to_string(),
            replacement: "".to_string(),
        }
    }

    #[test]
    fn test_mv_apply_leaves_element_unchanged_if_it_doesnt_apply() {
        let t = Transformation {
            kind: TransformationType::MV,
            matcher: "private_network".to_string(),
            replacement: "error".to_string(),
        };
        assert_eq!(element(), t.apply(&element()).element)
    }

    #[test]
    fn test_mv_apply_empty_command_if_it_doesnt_apply() {
        let t = Transformation {
            kind: TransformationType::MV,
            matcher: "private_network".to_string(),
            replacement: "error".to_string(),
        };
        assert_eq!(Command::NoOp, t.apply(&element()).command)
    }

    #[test]
    fn test_mv_apply_changes_element() {
        assert_eq!(
            Element::Resource("module.private_network.docker_network.network".to_string()),
            mv().apply(&element()).element
        )
    }

    #[test]
    fn test_mv_apply_creates_mv_command() {
        assert_eq!(
            Command::MV {
                source: element().to_string(),
                target: "module.private_network.docker_network.network".to_string()
            },
            mv().apply(&element()).command
        )
    }

    #[test]
    fn test_rm_apply_leaves_element_unchanged_if_it_doesnt_apply() {
        assert_eq!(element(), rm("private_network").apply(&element()).element)
    }

    #[test]
    fn test_rm_apply_empty_command_if_it_doesnt_apply() {
        assert_eq!(
            Command::NoOp,
            rm("private_network").apply(&element()).command
        )
    }

    #[test]
    fn test_rm_apply_removes_element() {
        assert_eq!(
            Element::Empty,
            rm("public_network").apply(&element()).element
        )
    }

    #[test]
    fn test_rm_apply_creates_rm_command() {
        assert_eq!(
            Command::RM("module.public_network.docker_network.network".to_string()),
            rm("public_network").apply(&element()).command
        )
    }
}
