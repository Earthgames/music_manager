use clap::{Parser, Subcommand};

//TODO add https://docs.rs/clap/4.1.4/clap/builder/enum.ValueHint.html
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// Clean tmp directory on exit
    #[clap(short, long)]
    pub clean: bool,

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
    /// Download youtube music and move in a genre directory
    #[clap(name = "down")]
    Download {
        url: String,
        #[clap(default_value_t = String::from("other"))]
        genre: String,
    },
    /// Print genres with a description
    #[clap(name = "genr")]
    Genres { genre: Option<String> },

    /// Makes a new genre directory
    #[clap(name = "mkgenr")]
    MakeGenre {
        genre: String,

        #[clap(default_value_t = String::from("default description, please insert your own"))]
        description: String,
    },
}
