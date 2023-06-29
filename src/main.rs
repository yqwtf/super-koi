pub mod chat;
pub mod commands;
pub mod config;
pub mod interface;

const SYSTEM_PROMPT: &str = "You are Koios, a chatbot running in a CLI.
YOU CAN RUN COMMANDS, use code blocks to do so
Use this to improve replies, access/modify local files and USE TOOLS LIKE CURL AND GREP
NEVER SAY YOU CAN'T ANSWER try your best
you only have access to the shell DON'T USE OTHER LANGUAGES

User: What's the weather here?

Koi: You can use curl to get the weather for your location with the following command
```
curl -s 'wttr.in/'$(curl -s ipinfo.io/city)'?format=%C:+%t'
```
ipinfo.io returns your location based on your IP address
wttr.in then takes your location and returns the weather";

#[tokio::main]
async fn main() -> Result<(), eventsource_client::Error> {
    interface::init();
    let api_key = config::parse();

    let mut messages: Vec<chat::Message> = [
        chat::Message {
            role: "system".to_string(),
            content: format!("Current OS: {}\n", std::env::consts::OS) + SYSTEM_PROMPT,
        },
        chat::Message {
            role: "user".to_string(),
            content: "find answers with the shell, always put runnable code in code blocks"
                .to_string(),
        },
    ]
    .to_vec();

    loop {
        let input = inquire::Text::new("").prompt();

        match input {
            Ok(input) => {
                messages.push(chat::Message {
                    role: "user".to_string(),
                    content: input.clone(),
                });

                if input == "exit" {
         