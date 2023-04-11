//NOTE - our main work will be done by calling an api with the query as body, for that lets first test how to call an api with reqwest library,
// lets try with the free dictonary api, https://api.dictionaryapi.dev/api/v2/entries/en/<word>

use clap::Parser;
use serde::{Deserialize, Serialize};

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

    println!("{:?}", results);
    Ok(())
    // println!("Query: {:?}", arguments.query);
}

// Ran successfully with
// ❯ cargo run -- -q "matter"
//    Compiling termoil v0.1.0 (/Users/ipriyam26/Programing/Rust/termoil)
// warning: structure field `partOfSpeech` should have a snake case name
//   --> src/main.rs:26:5
//    |
// 26 |     partOfSpeech: String,
//    |     ^^^^^^^^^^^^ help: convert the identifier to snake case: `part_of_speech`
//    |
//    = note: `#[warn(non_snake_case)]` on by default

// warning: `termoil` (bin "termoil") generated 1 warning
//     Finished dev [unoptimized + debuginfo] target(s) in 2.44s
//      Running `target/debug/termoil -q matter`
// [ApiResponse { word: "matter", phonetic: Some("/ˈmætə/"), phonetics: Some([Phonetic { text: "/ˈmætə/", audio: Some("") }, Phonetic { text: "/ˈmætɚ/", audio: Some("https://api.dictionaryapi.dev/media/pronunciations/en/matter-us.mp3") }]), origin: None, meanings: [Meaning { partOfSpeech: "noun", definitions: [Definition { definition: "Substance, material.", example: None, synonyms: Some([]), antonyms: Some([]) }, Definition { definition: "A condition, subject or affair, especially one of concern.", example: Some("What's the matter?; \u{a0} state matters"), synonyms: Some([]), antonyms: Some([]) }, Definition { definition: "An approximate amount or extent.", example: Some("I stayed for a matter of months."), synonyms: Some([]), antonyms: Some([]) }, Definition { definition: "The essence; the pith; the embodiment.", example: None, synonyms: Some([]), antonyms: Some([]) }, Definition { definition: "Inducing cause or reason, especially of anything disagreeable or distressing.", example: None, synonyms: Some([]), antonyms: Some([]) }, Definition { definition: "Pus.", example: None, synonyms: Some([]), antonyms: Some([]) }] }, Meaning { partOfSpeech: "verb", definitions: [Definition { definition: "To be important.", example: Some("Sorry for pouring ketchup on your clean white shirt! - Oh, don't worry, it does not matter."), synonyms: Some([]), antonyms: Some([]) }, Definition { definition: "(in negative constructions) To care about, to mind; to find important.", example: None, synonyms: Some([]), antonyms: Some([]) }, Definition { definition: "To form pus or matter, as an abscess; to maturate.", example: None, synonyms: Some([]), antonyms: Some([]) }] }] }]