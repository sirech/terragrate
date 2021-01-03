use std::fmt;

pub fn format_list<T: fmt::Display>(elements: &[T]) -> String {
    elements
        .iter()
        .map(|c| c.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .join("\n")
}
