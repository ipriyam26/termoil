//NOTE - our main work will be done by calling an api with the query as body, for that lets first test how to call an api with reqwest library,
// lets try with the free dictonary api, https://api.dictionaryapi.dev/api/v2/entries/en/<word>

use clap::Parser;


use std::error::Error;


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
    let results= b.json().await.expect("Error while parsing json");

    println!("{:?}", results);
    Ok(())
    // println!("Query: {:?}", arguments.query);
}

// Error -
// error[E0698]: type inside `async` block must be known in this context
//   --> src/main.rs:27:20
//    |
// 27 |     let results= b.json().await.expect("Error while parsing json");
//    |                    ^^^^ cannot infer type for type parameter `T` declared on the associated function `json`
//    |
// note: the type is part of the `async` block because of this `await`
//   --> src/main.rs:27:26
//    |
// 27 |     let results= b.json().await.expect("Error while parsing json");