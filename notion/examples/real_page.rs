//! Example: Load a real Notion page
//!
//! Usage:
//!   cargo run --example real_page -- YOUR_TOKEN
//!
//! This example loads the page at:
//! https://www.notion.so/287502506d2c800f9c00c9f8a5e285e3

use notion_re::api::{NotionClient, paths};
use notion_re::api::endpoints::LoadPageChunkRequest;
use notion_re::utils::extract_page_id_from_url;
use tracing::info;

const NOTION_URL: &str = "https://www.notion.so/287502506d2c800f9c00c9f8a5e285e3?v=287502506d2c814eb0c6000cbb508a4e";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let token = std::env::var("NOTION_TOKEN")
        .or_else(|_| {
            std::env::args()
                .nth(1)
                .ok_or_else(|| anyhow::anyhow!("No token provided. Pass token as argument or set NOTION_TOKEN env var"))
        })?;

    // Extract page ID from URL
    let page_id = extract_page_id_from_url(NOTION_URL)
        .expect("Could not extract page ID from URL");

    info!("Loading page: {} (ID: {})", NOTION_URL, page_id);

    let client = NotionClient::from_token(token);

    let load_request = LoadPageChunkRequest {
        page_id: page_id.clone(),
        chunk_number: 0,
        cur_cursor: None,
        vertical_columns: None,
    };

    let result = client.post(paths::LOAD_PAGE_CHUNK, &load_request).await;
    match result {
        Ok(response) => {
            info!("Page loaded successfully!");
            println!("{}", pretty_print_json(&response)?);
        }
        Err(e) => {
            info!("Failed to load page: {}", e);
            info!("Make sure your token is valid and you have access to this page.");
        }
    }

    Ok(())
}

fn pretty_print_json(json: &str) -> anyhow::Result<String> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    Ok(serde_json::to_string_pretty(&value)?)
}
