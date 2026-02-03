//! Basic usage example for Notion API reverse engineering
//!
//! To get your token:
//! 1. Open Notion in your browser
//! 2. Open Developer Tools (F12)
//! 3. Go to Application > Cookies > https://www.notion.so
//! 4. Copy the value of `token_v2` cookie
//!
//! Run with: cargo run --example basic_usage -- YOUR_TOKEN

use notion_re::api::{NotionClient, paths};
use serde_json::json;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Get token from command line or environment variable
    let token = std::env::var("NOTION_TOKEN")
        .or_else(|_| {
            std::env::args()
                .nth(1)
                .ok_or_else(|| anyhow::anyhow!("No token provided. Pass token as argument or set NOTION_TOKEN env var"))
        })?;

    let client = NotionClient::from_token(token);

    // Fetch user info
    info!("Fetching user info...");
    let empty_body = json!({});
    match client.post(paths::GET_USER, &empty_body).await {
        Ok(response) => {
            info!("User info received:");
            println!("{}", pretty_print_json(&response)?);
        }
        Err(e) => {
            info!("Failed to fetch user info: {}", e);
        }
    }

    // Fetch spaces
    info!("\nFetching spaces...");
    match client.post(paths::GET_SPACES, &empty_body).await {
        Ok(response) => {
            info!("Spaces received:");
            println!("{}", pretty_print_json(&response)?);
        }
        Err(e) => {
            info!("Failed to fetch spaces: {}", e);
        }
    }

    Ok(())
}

fn pretty_print_json(json: &str) -> anyhow::Result<String> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    Ok(serde_json::to_string_pretty(&value)?)
}
