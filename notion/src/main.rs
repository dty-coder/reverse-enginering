mod api;
mod models;
mod utils;

use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    info!("Notion API Reverse Engineering Tool");

    // TODO: Add your reverse engineering logic here

    Ok(())
}
