use clap::{Parser, Subcommand};
use music_manager::{clean_tmp, create_genre, download, genres};
use std::process;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Clean tmp directory on exit
    #[clap(short, long)]
    clean: bool,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// download youtube music and move in a genre directory
    #[clap(name = "down")]
    Download {
        url: String,
        #[clap(default_value_t = String::from("other"))]
        genre: String,
    },
    /// print genres with a discription
    #[clap(name = "genr")]
    Genres { genre: Option<String> },

    /// makes a new genre directory
    #[clap(name = "mkgenr")]
    MakeGenre {
        genre: String,

        #[clap(default_value_t = String::from("default discription, please insert your own"))]
        discription: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // download youtube music and move in a genre directo
        Commands::Download { url, genre } => match download(url, genre) {
            Ok(_t) => {
                if cli.clean {
                    if let Err(err) = clean_tmp() {
                        println!("could not clean temporary directory because: {}", err)
                    }
                }
            }
            Err(err) => {
                eprintln!("there was an error: {}", err);
                if cli.clean {
                    if let Err(err) = clean_tmp() {
                        println!("could not clean temporary directory because: {}", err)
                    }
                }
                process::exit(1);
            }
        },
        //print all genres with a discription
        Commands::Genres { genre } => match genres(genre) {
            Ok(_t) => (),
            Err(err) => {
                eprintln!("there was an error: {}", err);
                process::exit(1);
            }
        },

        Commands::MakeGenre { genre, discription } => create_genre(genre, discription).unwrap(),
    };
}
