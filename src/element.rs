use prettydiff::diff_chars;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Element {
    Resource(String),
    Empty,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Element::Resource(s) => s,
            Element::Empty => "",
        };
        write!(f, "{}", str)
    }
}

impl Element {
    pub fn diff(&self, other: &Element) -> String {
        let source = &self.to_string();
        let target = &other.to_string();
        diff_chars(source, target).to_string()
    }
}

#[cfg(test)]
mod element_tests {
    use super::*;

    #[test]
    fn test_diff_for_removal() {
        let diff =
            Element::Resource("docker_container.container".to_string()).diff(&Element::Empty);

        assert_eq!(diff, "\u{1b}[9;31mdocker_container.container\u{1b}[0m")
    }

    #[test]
    fn test_diff_for_additive_change() {
        let diff = Element::Resource("public_network.docker_network.network".to_string()).diff(
            &Element::Resource("module.public_network.docker_network.network".to_string()),
        );

        assert_eq!(
            diff,
            "\u{1b}[32mmodule.\u{1b}[0mpublic_network.docker_network.network"
        )
    }

    #[test]
    fn test_diff_for_destructive_change() {
        let diff = Element::Resource("module.public_network.docker_network.network".to_string())
            .diff(&Element::Resource(
                "public_network.docker_network.new_network".to_string(),
            ));

        assert_eq!(diff, "\u{1b}[9;31mmodule.\u{1b}[0mpublic_network.docker_network.ne\u{1b}[32mw_ne\u{1b}[0mtwork")
    }
}
