use std::fmt;

pub fn format_list<T: fmt::Display>(elements: &Vec<T>) -> String {
    elements
        .iter()
        .map(|c| c.to_string())
        .filter(|s| s != "")
        .collect::<Vec<String>>()
        .join("\n")
}
