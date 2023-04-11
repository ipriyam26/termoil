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


async fn main() {
    let arguments = Args::parse();
    let url = format!(
        "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
        arguments.query
    );
    let b = reqwest::get(url).await?;
    let results= b.json().await.expect("Error while parsing json");

    println!("{:?}", results);
  
    // println!("Query: {:?}", arguments.query);
}

// Error - error[E0277]: the `?` operator can only be used in an async function that returns `Result` or `Option` (or another type that implements `FromResidual`)
//   --> src/main.rs:25:36
//   |
// 19 |   async fn main() {
//   |  _________________-
// 20 | |     let arguments = Args::parse();
// 21 | |     let url = format!(
// 22 | |         "https://api.dictionaryapi.dev/api/v2/entries/en/{}",
// ...  |
// 25 | |     let b = reqwest::get(url).await?;
//   | |                                    ^ cannot use the `?` operator in an async function that returns `()`
// ...  |
// 30 | |     // println!("Query: {:?}", arguments.query);
// 31 | | }
//   | |_- this function should return `Result` or `Option` to accept `?`
//   |
//   = help: the trait `FromResidual<Result<Infallible, reqwest::Error>>` is not implemented for `()`

// error[E0752]: `main` function is not allowed to be `async`
//  --> src/main.rs:19:1
//   |
// 19 | async fn main() {
//   | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`