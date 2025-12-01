mod cli;
mod watcher;

use anyhow::Result;
use cli::client::{Cli, Commands};
use tracing::info;
use watcher::worker::Watcher;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::prs();
    match cli.command {
        Commands::Watch { path, detailed, recursive: _ } => {
            let path_trimmed = path.trim_matches('"').trim();
            info!("Watching {} detailed: {}", path_trimmed, detailed);
            let watcher = Watcher::new(path_trimmed.to_string(), detailed).await?;
            watcher.watch_entry(detailed).await?;
        }
        // Maybe add other commands later
        _ => unreachable!("???")
    }
    info!("Exiting...");
    Ok(())
}
