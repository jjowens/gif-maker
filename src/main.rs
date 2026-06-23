mod services;

use clap::{Parser, Subcommand};
use crate::services::makegif_service;
use crate::services::makegif_service::make_gif_256_colours;

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
        open_file_directory: String,
        #[clap(long, default_value = "test-output/test.gif")]
        save_gif_file_path: String,
    }
}

fn main() {
    println!("Gif-Maker");

    let args = Args::parse();

    match args.command {
        Some(Commands::MakeGif { open_file_directory, save_gif_file_path }) => {
            makegif_service::make_gif_alt(&open_file_directory, &save_gif_file_path).unwrap();
            make_gif_256_colours(&open_file_directory, &save_gif_file_path).unwrap();
        },
        None => (),
    }
}
