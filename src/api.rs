use std::{env, error::Error};

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub content: String,
}

pub async fn get_response(query: String, tokens: u32) -> Result<ApiResponse, Box<dyn Error>> {
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

pub fn get_body(query: String, tokens: u32) -> serde_json::Value {
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

pub fn get_header() -> HeaderMap<HeaderValue> {
    header::HeaderMap::from_iter(vec![
        (header::CONTENT_TYPE, "application/json".parse().unwrap()),
        (
            header::AUTHORIZATION,
            format!("Bearer {}", get_api_key()).parse().unwrap(),
        ),
    ])
}

pub fn get_api_key() -> String {
    env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY not set")
}
