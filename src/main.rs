mod services;

use clap::{Parser, Subcommand};
use crate::services::{gif_256_colours_service, makegif_service};

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
    },
    MakeCustom {
        #[clap(long)]
        open_file_directory: String,
        #[clap(long, default_value = "test-output/test.gif")]
        save_gif_file_path: String,
        #[clap(long, default_value_t = 100)]
        width: u16,
        #[clap(long, default_value_t = 100)]
        height: u16,
        #[clap(long, default_value = "")]
        colour_map: String,
    },
    MakeAlt {
        #[clap(long)]
        open_file_directory: String,
        #[clap(long, default_value = "test-output/test.gif")]
        save_gif_file_path: String,
    },
    Make256 {
        #[clap(long)]
        open_file_directory: String,
        #[clap(long, default_value = "test-output/make256Alt.gif")]
        save_gif_file_path: String,
    },
    Make256Alt {
        #[clap(long)]
        open_file_directory: String,
        #[clap(long, default_value = "test-output/make256.gif")]
        save_gif_file_path: String,
    },
    MakeBeacon {
        #[clap(long)]
        open_file_directory: String,
        #[clap(long, default_value = "test-output/make-beacon.gif")]
        save_gif_file_path: String,
    }

}

fn main() {
    println!("Gif-Maker");

    let args = Args::parse();

    match args.command {
        Some(Commands::MakeGif { open_file_directory, save_gif_file_path }) => {
            makegif_service::make_gif(&open_file_directory, &save_gif_file_path).unwrap();
        },
        Some(Commands::MakeCustom { open_file_directory, save_gif_file_path, width,height, colour_map }) => {
            makegif_service::clean_and_make_custom_gif(&open_file_directory, &save_gif_file_path, width, height, &colour_map).unwrap();
        },
        Some(Commands::MakeAlt { open_file_directory, save_gif_file_path }) => {
            makegif_service::make_gif_alt(&open_file_directory, &save_gif_file_path).unwrap();
        },
        Some(Commands::Make256 { open_file_directory, save_gif_file_path }) => {
            gif_256_colours_service::make_gif_256_colours(&open_file_directory, &save_gif_file_path).unwrap();
        },
        Some(Commands::Make256Alt { open_file_directory, save_gif_file_path }) => {
            gif_256_colours_service::make_gif_256_colours_alt(&open_file_directory, &save_gif_file_path).unwrap();
        },
        Some(Commands::MakeBeacon{ open_file_directory, save_gif_file_path }) => {
            gif_256_colours_service::make_beacon(&open_file_directory, &save_gif_file_path).unwrap();
        },
        None => (),
    }
}
