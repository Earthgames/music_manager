use clap::Parser;
use cli::{Cli, Commands};
use log::{self, error};
use music_manager::commands::*;
use simplelog::{LevelFilter, TermLogger};
use std::process;

mod cli;

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

        Commands::MakeGenre { genre, description } => match genre::create_genre(genre, description)
        {
            Ok(_t) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },
    };
}
