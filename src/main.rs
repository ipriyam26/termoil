//NOTE - our main work will be done by calling an api with the query as body, for that lets first test how to call an api with reqwest library,
// lets try with the free dictonary api, https://api.dictionaryapi.dev/api/v2/entries/en/<word>

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::error::Error;

#[derive(Deserialize, Debug,Serialize)]
struct ApiResponse {
    word: String,
    phonetic: Option<String>,
    phonetics: Option<Vec<Phonetic>>,
    origin: Option<String>,
    meanings: Vec<Meaning>,
}

#[derive(Deserialize, Debug,Serialize)]
struct Phonetic {
    text: String,
    audio: Option<String>,
}

#[derive(Deserialize, Debug,Serialize)]
struct Meaning {
    partOfSpeech: String,
    definitions: Vec<Definition>,
}

#[derive(Deserialize, Debug,Serialize)]
struct Definition {
    definition: String,
    example: Option<String>,
    synonyms: Option<Vec<String>>,
    antonyms: Option<Vec<String>>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // query the user wants to enter
    #[arg(short, long)]
    query: String,
}

//NOTE - Explain what is #[tokio::main] and why we need it
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Args::parse();
    let url = format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        arguments.query
    );
    let b = reqwest::get(url).await?;
    let results: Vec<ApiResponse> = b.json().await.expect("Error while parsing json");
    let pretty_results = to_string_pretty(&results[0].meanings).expect("Error while pretty-printing results");
    println!("{}", pretty_results);
    Ok(())
    // println!("Query: {:?}", arguments.query);
}

// Ran successfully
// ❯ cargo run -- -q "rust"
//    Compiling termoil v0.1.0 (/Users/ipriyam26/Programing/Rust/termoil)
// warning: structure field `partOfSpeech` should have a snake case name
//   --> src/main.rs:26:5
//    |
// 26 |     partOfSpeech: String,
//    |     ^^^^^^^^^^^^ help: convert the identifier to snake case: `part_of_speech`
//    |
//    = note: `#[warn(non_snake_case)]` on by default

// warning: `termoil` (bin "termoil") generated 1 warning
//     Finished dev [unoptimized + debuginfo] target(s) in 1.45s
//      Running `target/debug/termoil -q rust`
// [
//   {
//     "partOfSpeech": "noun",
//     "definitions": [
//       {
//         "definition": "The deteriorated state of iron or steel as a result of moisture and oxidation.",
//         "example": "The rust on my bicycle chain made cycling to work very dangerous.",
//         "synonyms": [],
//         "antonyms": []
//       },
//       {
//         "definition": "A similar substance based on another metal (usually with qualification, such as \"copper rust\").",
//         "example": "aerugo. Green or blue-green copper rust; verdigris. (American Heritage Dictionary, 1973)",
//         "synonyms": [],
//         "antonyms": []
//       },
//       {
//         "definition": "A reddish-brown color.",
//         "example": null,
//         "synonyms": [],
//         "antonyms": []
//       },
//       {
//         "definition": "A disease of plants caused by a reddish-brown fungus.",
//         "example": null,
//         "synonyms": [],
//         "antonyms": []
//       },
//       {
//         "definition": "Damage caused to stamps and album pages by a fungal infection.",
//         "example": null,
//         "synonyms": [],
//         "antonyms": []
//       }
//     ]
//   }
// ]