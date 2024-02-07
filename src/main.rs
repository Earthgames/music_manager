use clap::Parser;
use cli::{Cli, Commands};
use log::{self, error};
use music_manager::commands::*;
use simplelog::{LevelFilter, TermLogger};
use std::process;

mod cli;

fn main() {
    let cli = Cli::parse();
    let mut log_config = simplelog::ConfigBuilder::new();
    let mut quiet = false;
    TermLogger::init(
        match cli.loglevel {
            0 => {
                quiet = true;
                LevelFilter::Off
            }
            1 => LevelFilter::Error,
            2 => LevelFilter::Warn,
            3 => LevelFilter::Info,
            4 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        },
        log_config.set_time_level(LevelFilter::Off).build(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    match &cli.command {
        // download youtube music and move in a genre directory
        Commands::Download { url, genre } => match download::download(url, genre, quiet) {
            Ok(_) => {
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
            Ok(_) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },

        Commands::MakeGenre { genre, description } => match genre::create_genre(genre, description)
        {
            Ok(_) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },
    };
}
