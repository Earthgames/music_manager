use clap::{Parser, Subcommand, ValueHint};

//TODO add https://docs.rs/clap/4.1.4/clap/builder/enum.ValueHint.html
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// Log level:
    /// 0 quiet,
    /// 1 errors,
    /// 2 warnings,
    /// 3 info,
    #[clap(short, long)]
    #[clap(default_value_t = 3)]
    pub loglevel: u8,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download youtube music and move in a category directory
    #[clap(name = "down")]
    Download {
        /// Clean tmp directory on exit
        #[clap(short, long)]
        clean: bool,

        #[clap(value_hint=ValueHint::Url)]
        url: String,
        #[clap(default_value_t = String::from("other"))]
        category: String,
    },
    /// Print categories with a description
    #[clap(name = "genr")]
    Categories { category: Option<String> },

    /// Makes a new category directory
    #[clap(name = "mkgenr")]
    MakeCategory {
        category: String,

        #[clap(default_value_t = String::from("default description, please insert your own"))]
        description: String,
    },

    /// Add music to library
    #[clap(name = "add")]
    AddToLib {
        #[clap(value_hint=ValueHint::FilePath)]
        files: String,

        #[clap(default_value_t = String::from("other"))]
        category: String,
    },
}
