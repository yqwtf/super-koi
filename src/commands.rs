use console::{style, Style};

#[must_use]
pub fn parse(input: &str) -> Vec<String> {
    let mut responses: Vec<String> = Vec::new();

    if input.contains("```") {
        let command_blocks = input
       