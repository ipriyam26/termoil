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


 fn main(){
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

//ERROR -
// warning: unused import: `std::error::Error`
//  --> src/main.rs:7:5
//   |
// 7 | use std::error::Error;
//   |     ^^^^^^^^^^^^^^^^^
//   |
//   = note: `#[warn(unused_imports)]` on by default

// error[E0728]: `await` is only allowed inside `async` functions and blocks
//   --> src/main.rs:25:30
//    |
// 19 |  fn main(){
//    |     ---- this is not `async`
// ...
// 25 |     let b = reqwest::get(url).await?;
//    |                              ^^^^^^ only allowed inside `async` functions and blocks

// error[E0728]: `await` is only allowed inside `async` functions and blocks
//   --> src/main.rs:26:26
//    |
// 19 |  fn main(){
//    |     ---- this is not `async`
// ...
// 26 |     let results= b.json().await.expect("Error while parsing json");
//    |                          ^^^^^^ only allowed inside `async` functions and blocks

// error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
//   --> src/main.rs:25:36
//    |
// 19 |  fn main(){
//    |  --------- this function should return `Result` or `Option` to accept `?`
// ...
// 25 |     let b = reqwest::get(url).await?;
//    |                                    ^ cannot use the `?` operator in a function that returns `()`
//    |
//    = help: the trait `FromResidual<Result<Infallible, reqwest::Error>>` is not implemented for `()`

