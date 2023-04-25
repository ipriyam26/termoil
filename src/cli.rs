use clap::{command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Config {
        #[arg(short = 't', group = "commands")]
        tokens: Option<u32>,
        // display all the information about the system collected
        #[arg(short = 'd', long = "display", group = "commands")]
        display: bool,
    },
    #[command(about = "Search for a command")]
    Search {
        query: String,
        #[arg(short = 't')]
        tokens: Option<u32>,
    },
}
