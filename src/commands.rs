use console::{style, Style};

#[must_use]
pub fn parse(input: &str) -> Vec<String> {
    let mut responses: Vec<String> = Vec::new();

    if input.contains("```") {
        let command_blocks = input
            .split("```")
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, block)| block)
            .collect::<Vec<&str>>();

        for block in command_blocks {
            let block = block
                .lines()
                .skip(1)
                .take_while(|line| line != &"")
                .collect::<Vec<&str>>()
                .join("\n");

            let output = run_checked(block.to_string());

            if output == "Stop Command Run" {
                break;
            }

      