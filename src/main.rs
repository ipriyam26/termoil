//NOTE - We should now try with the actual service we would like to use it in, so lets check the open ai api.

use clap::Parser;
use serde_json::to_string_pretty;
use std::{error::Error, env};
use dotenv::dotenv;


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

   
    Ok(())
    // println!("Query: {:?}", arguments.query);
}

