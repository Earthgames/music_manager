use std::process;

use clap::Parser;
use log::{self, error, info};
use simplelog::{LevelFilter, TermLogger};

use cli::{Cli, Commands};
use music_manager::commands::*;

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
        // download YouTube music and move in a category directory
        Commands::Download { url, category } => match down::download(url, category, quiet) {
            Ok(_) => {
                process::exit(0);
            }
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },
        // print all categories with a description
        Commands::Categories { category } => match cat::category(category) {
            Ok(_) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },

        Commands::MakeCategory {
            category,
            description,
        } => match cat::mk_category(category, description) {
            Ok(_) => process::exit(0),
            Err(err) => {
                error!("{err}");
                process::exit(1);
            }
        },

        Commands::AddToLib {
            files,
            category,
            force,
        } => {
            // check if we get files
            if files.is_empty() {
                info!("No files provided");
                process::exit(0)
            }

            match add::add(files, category, quiet, *force) {
                Ok(_) => process::exit(0),
                Err(err) => {
                    error!("{err}");
                    process::exit(1);
                }
            }
        }
    };
}
