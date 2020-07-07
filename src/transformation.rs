use crate::element::Element;
use serde_derive::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub enum TransformationType {
    MV,
    RM,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Transformation {
    pub kind: TransformationType,
    pub matcher: String,
    pub replacement: String,
}

impl Transformation {
    pub fn apply(&self, e: &Element) -> Element {
        match self.kind {
            TransformationType::MV => self.mv(e),
            TransformationType::RM => self.rm(e),
        }
    }

    fn mv(&self, e: &Element) -> Element {
        match e {
            Element::Resource(r) => Element::Resource(r.replace(&self.matcher, &self.replacement)),
            Element::Empty => Element::Empty,
        }
    }

    fn rm(&self, e: &Element) -> Element {
        match e {
            Element::Resource(r) => {
                if r.contains(&self.matcher) {
                    Element::Empty
                } else {
                    e.clone()
                }
            }
            Element::Empty => Element::Empty,
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
        assert_eq!(e, t.apply(&e))
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
            t.apply(&e)
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
        assert_eq!(e, t.apply(&e))
    }

    #[test]
    fn test_rm_apply_removes_element() {
        let e = Element::Resource("module.public_network.docker_network.network".to_string());
        let t = Transformation {
            kind: TransformationType::RM,
            matcher: "public_network".to_string(),
            replacement: "".to_string(),
        };
        assert_eq!(Element::Empty, t.apply(&e))
    }
}
