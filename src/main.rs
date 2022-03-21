use clap::{Parser, Subcommand};
use music_manager_helper::{download, genres};
use std::process;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// download youtube music and move in a genre directory
    Download {
        url: String,
        #[clap(default_value_t = String::from("other"))]
        genre: String,
    },
    /// print genres with a discription
    Genres { genre: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // download youtube music and move in a genre directo
        Commands::Download { url, genre } => match download(url, genre) {
            Ok(_t) => (),
            Err(err) => {
                eprintln!("there was an error while running mode downloading: {}", err);
                process::exit(1);
            }
        },
        //print all genres with a discription
        Commands::Genres { genre } => match genres(genre) {
            Ok(_t) => (),
            Err(err) => {
                eprintln!("there was an error while running mode genres: {}", err);
                process::exit(1);
            }
        },
    }
    // };
}
