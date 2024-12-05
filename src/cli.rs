use clap::{Parser, Subcommand, ValueHint};

//TODO: add https://docs.rs/clap/4.1.4/clap/builder/enum.ValueHint.html
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

    /// Is the same as log level 0
    /// Will override the log level
    #[clap(short, long)]
    pub quiet: bool,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Download YouTube music and move in a category directory
    #[clap(name = "down")]
    Download {
        #[clap(value_hint=ValueHint::Url)]
        url: String,
        #[clap(default_value_t = String::from("other"))]
        category: String,
    },

    /// Add music to library
    #[clap(name = "add")]
    AddToLib {
        /// Force repay tags to be recalculated
        #[clap(short, long)]
        force: bool,
        #[clap(short, long)]
        singles: bool,
        #[clap(value_hint=ValueHint::FilePath)]
        files: Vec<String>,

        #[clap(short, long)]
        category: String,
    },

    /// Print categories with a description
    #[clap(name = "cat")]
    Categories { category: Option<String> },

    /// Makes a new category directory
    #[clap(name = "mkcat")]
    MakeCategory {
        category: String,

        #[clap(default_value_t = String::from("default description, please insert your own"))]
        description: String,
    },

    /// Check music
    #[clap(name = "check")]
    Check {
        /// The category that needs to be checked, optional
        category: Option<String>,

        /// If the tags and path need to be checked, this is a lot slower
        #[clap(short, long)]
        tags_path: bool,
    },

    /// Tag music and move to the library
    #[clap(name = "tag")]
    Tag {
        /// Force files that are tagged to be tagged
        #[clap(short, long)]
        force: bool,

        #[clap(value_hint=ValueHint::FilePath)]
        files: Vec<String>,

        /// The category that the tagged file will be moved to
        #[clap(short, long)]
        category: String,
    },
}
