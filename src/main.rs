mod services;

use clap::{Parser, Subcommand};
use crate::Commands::MakeGif;

#[derive(Parser)]
#[command(name = "appname", author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    MakeGif {
        #[clap(long)]
        openFileDirectory: String,
        #[clap(long, default_value = "test-output")]
        saveGifFilePath: String,
    }
}

fn main() {
    println!("Gif-Maker");

    let args = Args::parse();

    match args.command {
        Some(Commands::MakeGif { openFileDirectory, saveGifFilePath }) => {
            println!("Making GIF");
        },
        None => (),
    }
}
