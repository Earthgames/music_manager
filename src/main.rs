use std::env::current_dir;

use clap::Parser;
use log::{self, info};
use simplelog::{LevelFilter, TermLogger};

use cli::{Cli, Commands};
use music_manager::commands::*;
use music_manager::tag;

use anyhow::Result;

mod cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut log_config = simplelog::ConfigBuilder::new();
    let mut quiet = false;
    TermLogger::init(
        if cli.quiet {
            quiet = true;
            LevelFilter::Off
        } else {
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
            }
        },
        log_config.set_time_level(LevelFilter::Off).build(),
        simplelog::TerminalMode::Stdout,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    match &cli.command {
        // download YouTube music and move in a category directory
        Commands::Download { url, category } => down::download(url, category, &quiet)
        ,
        // print all categories with a description
        Commands::Categories { category } => cat::category(category),

        Commands::MakeCategory {
            category,
            description,
        } => cat::mk_category(category, description),

        Commands::AddToLib {
            files,
            category,
            force,
            singles,
        } => {
            // check if we get files
            if files.is_empty() {
                info!("No files provided");
                return Ok(());
            }

            add::add(files, category, &quiet, force, &!singles)
        }
        Commands::Check {
            category,
            tags_path,
        } => check::check(category, tags_path),
        Commands::Tag {
            category,
            files,
            force,
        } => tag::tag(current_dir().unwrap(), files, category, &quiet, force),
    }
}
