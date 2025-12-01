mod cli;
mod watcher;

use watcher::ignore::Ignore;

use anyhow::Result;
use cli::client::{Cli, Commands};
use tracing::info;
use watcher::worker::Watcher;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::prs();
    match cli.command {
        Commands::Watch { path, detailed, ignore_list, recursive: _ } => {
            let path_trimmed = path.trim_matches('"').trim();
            info!("Watching {} detailed: {}", path_trimmed, detailed);

            let ignore = if let Some(ignore_list) = ignore_list {
                Some(Ignore::from(ignore_list.as_str())?)
            } else {
                None
            };
            let watcher = Watcher::new(path_trimmed.to_string(), detailed).await?;
            watcher.watch_entry(detailed, ignore).await?;
        }
        // Maybe add other commands later
        _ => unreachable!("???")
    }
    info!("Exiting...");
    Ok(())
}
