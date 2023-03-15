use colored::Colorize;
use regex::Regex;
use std::{env, error::Error};

pub struct Config {
    pub query: String,
    pub openai_api_key: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, Box<dyn Error>> {
        // Convert all the args after 0 into a string
        let query: String = args[1..].join(" ");
        let openai_api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

        Ok(Config {
            query,
            openai_api_key,
        })
    }
}

// Define struct for {"role": "user", "content": "Hello"} and {"role": "assistant", "content": "Hello"}
struct Message {
    role: String,
    content: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut messages: Vec<Message> = vec![Message {
        role: "user".to_string(),
        content: config.query.clone(),
    }];

    let openai_resp = call_openai(&messages, &config);

    // Respond to initial query
    print_bot_response(&openai_resp, true);

    // Add to messages
    messages.push(Message {
        role: "assistant".to_string(),
        content: openai_resp,
    });

    loop {
        print_user_prompt();

        // Wait for the user to respond
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;

        // Remove trailing newline
        user_input = user_input.trim().to_string();

        // Check if user wants to quit
        if user_input == "quit" {
            println!("{}", "PEACE!".bold().green());
            break;
        }

        // Add to messages
        messages.push(Message {
            role: "user".to_string(),
            content: user_input.clone(),
        });

        // Call OpenAI again
        let openai_resp = call_openai(&messages, &config);

        // Respond to user's response
        print_bot_response(&openai_resp, false);
    }

    Ok(())
}

fn print_user_prompt() {
    println!("{}: ", "You".bold().blue());
}

fn print_bot_response(resp: &str, show_instructions: bool) {
    println!(
        "\n{}: {}{}",
        "rustpal".bold().purple(),
        resp,
        if !show_instructions { "\n" } else { "" }
    );

    if show_instructions {
        println!(
            "{}\n",
            "Type 'quit' to exit, or press Ctrl+C to quit at any time."
                .italic()
                .dimmed()
        );
    }
}

fn call_openai(messages: &Vec<Message>, config: &Config) -> String {
    let resp: String = ureq::post("https://api.openai.com/v1/chat/completions")
        .set("Content-Type", "application/json")
        .set(
            "Authorization",
            &format!("Bearer {}", &config.openai_api_key),
        )
        .send_json(ureq::json!({
          "model": "gpt-3.5-turbo",
          "messages": messages.iter().map(|m| {
              ureq::json!({
                  "role": m.role,
                  "content": m.content,
              })
          }).collect::<Vec<serde_json::Value>>()
        }))
        .unwrap()
        .into_string()
        .unwrap();

    let json: serde_json::Value = serde_json::from_str(&resp).unwrap();

    // Convert to JSON and return 'message' key if it exists.
    // Otherwise, return "Sorry, rustpal is not available right now"
    if json["choices"][0]["message"]["content"].is_string() {
        let re = Regex::new(r"^[\n]+").unwrap();

        // Remove quotes from the response, and strip leading and trailing line breaks
        return re
            .replace(
                &json["choices"][0]["message"]["content"]
                    .to_string()
                    .replace("\"", "")
                    .to_string()
                    .replace(r"\n", "\n")
                    .to_string(),
                "",
            )
            .to_string();
    } else {
        return "Sorry, rustpal is not available right now".to_string();
    }
}
