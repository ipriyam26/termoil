//NOTE - lets allow the user to provide max_tokens

use clap::{command, Parser, Subcommand};

use dotenv::dotenv;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use serde_json::json;
use std::{
    env::{self, consts::OS},
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Debug, Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Config,
    #[command(about = "Search for a command")]
    Search {
        query: String,
        #[arg(short = 't')]
        tokens: Option<u32>,
    },
}

fn get_pretty_name() -> io::Result<String> {
    match OS {
        "linux" => {
            let os_release_path = Path::new("/etc/os-release");
            let file = File::open(&os_release_path)?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if line.starts_with("PRETTY_NAME=") {
                    return Ok(line[13..line.len() - 1].to_string());
                }
            }
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "PRETTY_NAME not found in /etc/os-release",
            ))
        }
        "windows" => Ok("Windows".to_string()),
        "macos" => Ok("macOS".to_string()),
        _ => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Unknown operating system",
        )),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let arguments = Args::parse();

    match arguments.command {
        Commands::Config => todo!(),
        Commands::Search { tokens, query } => {
            // if tokens is None make it 200 by default
            let tokens = tokens.unwrap_or(200);
            let response: ApiResponse = get_response(query, tokens).await?;
            println!("{:?}", &response.choices[0]);
        }
    }

    Ok(())
    // println!("Query: {:?}", arguments.query);
}

fn get_api_key() -> String {
    env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY not set")
}

fn get_os() -> String {
    get_pretty_name().unwrap_or("Linux".to_owned())
}

fn get_header() -> HeaderMap<HeaderValue> {
    header::HeaderMap::from_iter(vec![
        (header::CONTENT_TYPE, "application/json".parse().unwrap()),
        (
            header::AUTHORIZATION,
            format!("Bearer {}", get_api_key()).parse().unwrap(),
        ),
    ])
}

fn get_system_message() -> String {
    format!(
        "Act as a terminal expert, answer should be the COMMAND ONLY, no need to explain. OS: {OS}",
        OS = get_os()
    )
}

fn get_body(query: String, tokens: u32) -> serde_json::Value {
    json!(
        {
            "model":"gpt-3.5-turbo",
            "messages":[
                {"role": "system",
                "content": get_system_message()
                },
            {
                "role":"user",
                "content": query,
            }
            ],
            "max_tokens": tokens,
        }
    )
}

async fn get_response(query: String, tokens: u32) -> Result<ApiResponse, Box<dyn Error>> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let response: ApiResponse = client
        .post(url)
        .headers(get_header())
        .json(&get_body(query, tokens))
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
