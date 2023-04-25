//NOTE - Even though in tests its working correctly, Its possible that the response maybe not in format as expected so we will resend the request again
mod cli;
use clap::Parser;
use cli::{Args, Commands};
use dotenv::dotenv;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env::{self, consts::OS},
    error::Error,
    fs::{File, OpenOptions},
    io::{self, BufRead, Write},
    path::Path,
    process::Command,
};

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


fn get_pretty_name() -> io::Result<String> {
    match OS {
        "linux" => {
            let os_release_path = Path::new("/etc/os-release");
            let file = File::open(os_release_path)?;
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

fn get_api_key() -> String {
    env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY not set")
}

fn get_default_tokens() -> u32 {
    env::var("TOKENS")
        .unwrap_or("350".to_owned())
        .parse()
        .expect("tokens should be a number")
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
    "Act as a smart terminal assistant API server, provide   help with general tasks you are allowed to use external dependencies, here is the example output format, output should be in JSON:\n
    {
    \"instruction_commands\": [\"ffmpeg -i input_video.mp4 -vn -acodec copy output_audio.m4a\"],
    \"external_commands\": [\"ffmpeg\"],
    \"external_install\": [\"sudo apt install ffmpeg\"],
    \"explanation\": \"| Part | Description |\n| --- | --- |\n| Command | ffmpeg -i input_video.mp4 -vn -acodec copy output_audio.m4a |\n| ffmpeg | A command-line tool used for handling audio, video, and other multimedia files. |\n| -i input_video.mp4 | Specifies the input video file. |\n| -vn | Disables the video stream from the output. |\n| -acodec copy | Copies the audio stream from the input file to the output file without any re-encoding. |\n| output_audio.m4a | Specifies the output audio file. The format of the output audio file is determined by its extension, which in this case is .m4a. |\"
    }
    \n
    The output should be a JSON object with the following fields:\n
    instruction_commands: A list of commands that can be run to complete the task.\n
    external_commands: A list of commands that are not built-in to the terminal, but are required to complete the task.\n
    external_install: A list of commands that can be run to install the external commands.\n
    explanation: A markdown table that explains the commands and their arguments.\n
    \n
    Here is your first task: 
    ".to_owned()
}

fn get_body(query: String, tokens: u32) -> serde_json::Value {
    json!(
        {
            "model":"gpt-3.5-turbo",
            "messages":[
                // {"role": "system",
                // "content": get_system_message()
                // },
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
    let body = &get_body(query, tokens);
    let response: ApiResponse = client
        .post(url)
        .headers(get_header())
        .json(body)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
