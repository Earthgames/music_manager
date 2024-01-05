use clap::{Parser, Subcommand};
use log::{self, error};
use music_manager::commands::*;
use simplelog::{LevelFilter, TermLogger};
use std::process;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    /// Clean tmp directory on exit
    #[clap(short, long)]
    clean: bool,

    /// log level:
    /// 0 silent,
    /// 1 errors,
    /// 2 warnings,
    /// 3 info,
    #[clap(short, long)]
    #[clap(default_value_t = 3)]
    loglevel: u8,

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
    /// print genres with a description
    #[clap(name = "genr")]
    Genres { genre: Option<String> },

    /// makes a new genre directory
    #[clap(name = "mkgenr")]
    MakeGenre {
        genre: String,

        #[clap(default_value_t = String::from("default description, please insert your own"))]
        description: String,
    },
}

fn main() {
    let cli = Cli::parse();
    TermLogger::init(
        match cli.loglevel {
            0 => LevelFilter::Off,
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        },
        simplelog::Config::default(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    match &cli.command {
        // download youtube music and move in a genre directory
        Commands::Download { url, genre } => match download::download(url, genre) {
            Ok(_t) => {
                if cli.clean {
                    clean_tmp()
                }
                process::exit(0);
            }
            Err(err) => {
                error!("{err}");
                if cli.clean {
                    clean_tmp()
                }
                process::exit(1);
            }
        },
        // print all genres with a description
        Commands::Genres { genre } => match genre::genres(genre) {
            Ok(_t) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },

        Commands::MakeGenre { genre, description } => match genre::create_genre(genre, description) {
            Ok(_t) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },
    };
}
