//NOTE - even though std library option is good enough its not the best for CLI apps and lot of things will have to be done manually so we will be using a cargo crate called clap

use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // query the user wants to enter
    #[arg(short, long)]
    query: String,
}


fn main(){
    let arguments = Args::parse();
    println!("Query: {:?}", arguments.query);
}
