//NOTE - We should now try with the actual service we would like to use it in, so lets check the open ai api.

use clap::Parser;
use dotenv::dotenv;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use serde_json::{json};
use std::{env, error::Error};

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
    // query the user wants to enter
    #[arg(short, long)]
    query: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let arguments = Args::parse();
    let open_ai_api_key = env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY not set");
    let query = arguments.query.to_owned();
    let client = Client::new();

    let url = "https://api.openai.com/v1/chat/completions";

    let headers: HeaderMap<HeaderValue> = header::HeaderMap::from_iter(vec![
        (header::CONTENT_TYPE, "application/json".parse().unwrap()),
        (
            header::AUTHORIZATION,
            format!("Bearer {}", open_ai_api_key).parse().unwrap(),
        ),
    ]);

    let body = json!(
        {
            "model":"gpt-3.5-turbo",
            "messages":[{
                "role":"user",
                "content": query,
            }]
        }
    );
    // println!("{:#?}", &body);

    let response: ApiResponse = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;


    println!("{}", &response.choices[0].message.content);

    Ok(())
    // println!("Query: {:?}", arguments.query);
}
