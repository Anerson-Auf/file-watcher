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
        Commands::Watch { path, detailed, recursive } => {
            info!("Watching {} detailed: {}", path, detailed);
            let watcher = Watcher::new(path, detailed).await?;
            watcher.watch_entry(detailed).await?;
        }
        _ => unreachable!("???")
    }
    info!("Exiting...");
    Ok(())
}
