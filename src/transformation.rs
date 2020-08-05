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
        let new_element = match e {
            Element::Resource(r) => {
                if r.contains(&self.matcher) {
                    Element::Empty
                } else {
                    e.clone()
                }
            }
            Element::Empty => Element::Empty,
        };

        TransformationResult {
            element: new_element,
            command: Command::NoOp,
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::*;

    #[test]
    fn test_mv_apply_leaves_element_unchanged_if_it_doesnt_apply() {
        let e = Element::Resource("module.public_network.docker_network.network".to_string());
        let t = Transformation {
            kind: TransformationType::MV,
            matcher: "private_network".to_string(),
            replacement: "error".to_string(),
        };
        assert_eq!(e, t.apply(&e).element)
    }

    #[test]
    fn test_mv_apply_empty_command_if_it_doesnt_apply() {
        let e = Element::Resource("module.public_network.docker_network.network".to_string());
        let t = Transformation {
            kind: TransformationType::MV,
            matcher: "private_network".to_string(),
            replacement: "error".to_string(),
        };
        assert_eq!(Command::NoOp, t.apply(&e).command)
    }

    #[test]
    fn test_mv_apply_changes_element() {
        let e = Element::Resource("module.public_network.docker_network.network".to_string());
        let t = Transformation {
            kind: TransformationType::MV,
            matcher: "public_network".to_string(),
            replacement: "private_network".to_string(),
        };
        assert_eq!(
            Element::Resource("module.private_network.docker_network.network".to_string()),
            t.apply(&e).element
        )
    }

    #[test]
    fn test_mv_apply_creates_mv_command() {
        let e = Element::Resource("module.public_network.docker_network.network".to_string());
        let t = Transformation {
            kind: TransformationType::MV,
            matcher: "public_network".to_string(),
            replacement: "private_network".to_string(),
        };
        assert_eq!(
            Command::MV {
                source: e.to_string(),
                target: "module.private_network.docker_network.network".to_string()
            },
            t.apply(&e).command
        )
    }

    #[test]
    fn test_rm_apply_leaves_element_unchanged_if_it_doesnt_apply() {
        let e = Element::Resource("module.public_network.docker_network.network".to_string());
        let t = Transformation {
            kind: TransformationType::RM,
            matcher: "private_network".to_string(),
            replacement: "".to_string(),
        };
        assert_eq!(e, t.apply(&e).element)
    }

    #[test]
    fn test_rm_apply_removes_element() {
        let e = Element::Resource("module.public_network.docker_network.network".to_string());
        let t = Transformation {
            kind: TransformationType::RM,
            matcher: "public_network".to_string(),
            replacement: "".to_string(),
        };
        assert_eq!(Element::Empty, t.apply(&e).element)
    }
}
