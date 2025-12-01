use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn prs() -> Self {
        Parser::parse()
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start watcher
    Watch {
        /// Path to watch. Can be a file or a directory.
        #[arg(short, long)]
        path: String,
        /// Detailed output of events.
        #[arg(short, long)]
        detailed: bool,
        /// Path to ignore list file.
        #[arg(short, long)]
        ignore_list: Option<String>,
        /// Not it's usefull for me, but it's here
        #[arg(short, long)]
        recursive: bool,
    }
}

