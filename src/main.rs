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
        // download youtube music and move in a category directory
        Commands::Download { url, category } => match download::download(url, category, quiet) {
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
        // print all categories with a description
        Commands::Categories { category } => match category::category(category) {
            Ok(_) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },

        Commands::MakeCategory {
            category,
            description,
        } => match category::mk_category(category, description) {
            Ok(_) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },

        Commands::AddToLib { files, category } => match add::add(files, category, quiet) {
            Ok(_) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },
    };
}
