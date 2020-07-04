use serde_derive::Deserialize;

#[derive(Clone, PartialEq, Debug)]
enum Element {
    Resource(String),
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Transformation {
    pub matcher: String,
    pub replacement: String,
}

impl Transformation {
    fn apply(self, e: &Element) -> Element {
        match e {
            Element::Resource(r) => Element::Resource(r.replace(&self.matcher, &self.replacement)),
        }
    }
}

#[cfg(test)]
mod transform_tests {
    use super::*;

    #[test]
    fn test_apply_leaves_element_unchanged_if_it_doesnt_apply() {
        let e = Element::Resource("module.public_network.docker_network.network".to_string());
        let t = Transformation {
            matcher: "private_network".to_string(),
            replacement: "error".to_string(),
        };
        assert_eq!(e, t.apply(&e))
    }

    #[test]
    fn test_apply_changes_element() {
        let e = Element::Resource("module.public_network.docker_network.network".to_string());
        let t = Transformation {
            matcher: "public_network".to_string(),
            replacement: "private_network".to_string(),
        };
        assert_eq!(
            Element::Resource("module.private_network.docker_network.network".to_string()),
            t.apply(&e)
        )
    }
}
