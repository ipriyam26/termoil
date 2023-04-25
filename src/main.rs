//NOTE - Even though in tests its working correctly, Its possible that the response maybe not in format as expected so we will resend the request again
mod api;
mod cli;
mod os;
use api::{get_response, ApiResponse};
use clap::Parser;
use cli::{Args, Commands};
use dotenv::dotenv;

use os::get_system_message;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{OpenOptions},
    io::{Write},
    process::Command,
};

use crate::os::{get_os, get_default_tokens};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let arguments = Args::parse();

    match arguments.command {
        Commands::Config { tokens, display } => {
            if tokens.is_some() {
                // store the tokens in the .env file that is created by the dotenv crate
                OpenOptions::new()
                    .append(true)
                    .open(".env")?
                    .write_all(format!("TOKENS={}", tokens.unwrap()).as_bytes())?;
            }
            if display {
                println!("OS: {}", get_os());
                println!("Tokens: {}", get_default_tokens());
            }
        }
        Commands::Search { tokens, mut query } => {
            let tokens = tokens.unwrap_or(get_default_tokens());
            query = format!(
                "{system}{query} using terminal OS: {OS}",
                system = get_system_message(),
                query = query,
                OS = get_os()
            );

            handle_request(query, tokens).await?;
        }
    }

    Ok(())
    // println!("Query: {:?}", arguments.query);
}

async fn handle_request(query: String, tokens: u32) -> Result<(), Box<dyn Error>> {
    let mut command = None;
    for _ in 0..3 {
        let response: ApiResponse = get_response(query.clone(), tokens).await?;
        if let Ok(parsed_command) =
            serde_json::from_str::<Instructions>(&response.choices[0].message.content)
        {
            command = Some(parsed_command);
            break;
        }
    }

    match command {
        Some(command) => {
            handle_external_commands(&command);
            println!("{}", command.instruction_commands[0]);
        }
        None => {
            println!("Error in parsing the response, Please try again with a different query");
        }
    }
    Ok(())
}

fn handle_external_commands(command: &Instructions) {
    let mut found_one = false;
    // check the list of external commands and if any of them is not installed, print the list of commands to install them
    command
        .external_commands
        .iter()
        .enumerate()
        .for_each(|(index, tool)| {
            let output = Command::new("which").arg(tool.trim()).output();
            if let Ok(output) = output {
                if !output.status.success() {
                    if !found_one {
                        println!("Run the following commands to install the required tools:");
                        found_one = true;
                    }
                    println!(
                        "{}",
                        command
                            .external_install
                            .get(index)
                            .expect("Index out of bounds")
                    );
                }
            }
        });
}

#[derive(Debug, Deserialize, Serialize)]
struct Instructions {
    instruction_commands: Vec<String>,
    external_commands: Vec<String>,
    external_install: Vec<String>,
    explanation: String,
}


