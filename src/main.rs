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
        saveGifDir: String,
        #[clap(long, default_value = "test.gif")]
        saveGifFilename: String
    }
}

fn main() {
    println!("Gif-Maker");

    let args = Args::parse();

    match args.command {
        Some(Commands::MakeGif { openFileDirectory, saveGifDir, saveGifFilename }) => {
            println!("Making GIF");
        },
        None => (),
    }
}
