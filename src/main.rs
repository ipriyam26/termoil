use clap::Parser;
use serde::Deserialize;

// use serde::Deserialize;

#[derive(Deserialize)]
struct Quote {
    quote: String,
    character: String,
    anime: String,
}
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    query: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let request_url = "https://animechan.vercel.app/api/random";

    let response = reqwest::get(request_url)
        .await
        .expect("Failed to get response");


    //parse the  response into a Quote struct
    let quote: Quote = response.json().await.expect("Failed to parse response");

    println!("{} - {}", quote.quote, quote.character);

    println!("Hello, {}!", args.query);
}
