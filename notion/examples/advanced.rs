//! Advanced example: Load a Notion page and search
//!
//! Usage:
//!   cargo run --example advanced -- YOUR_TOKEN PAGE_ID
//!
//! Get PAGE_ID from a Notion URL:
//!   https://notion.so/username/Page-Name-1234567890abcdef1234567890abcdef
//!   The ID is: 1234567890abcdef1234567890abcdef

use notion_re::api::{NotionClient, paths};
use notion_re::api::endpoints::{LoadPageChunkRequest, SearchRequest};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        anyhow::bail!("Usage: {} <TOKEN> <PAGE_ID>", args[0]);
    }

    let token = &args[1];
    let page_id = &args[2];

    let client = NotionClient::from_token(token.clone());

    // Load a page chunk
    info!("Loading page: {}", page_id);
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
        }
    }

    // Search for content
    info!("\nSearching for 'test'...");
    let search_request = SearchRequest {
        query: "test".to_string(),
        ancestor_id: None,
        space_id: None,
        limit: Some(10),
    };

    let result = client.post(paths::SEARCH, &search_request).await;
    match result {
        Ok(response) => {
            info!("Search results:");
            println!("{}", pretty_print_json(&response)?);
        }
        Err(e) => {
            info!("Search failed: {}", e);
        }
    }

    Ok(())
}

fn pretty_print_json(json: &str) -> anyhow::Result<String> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    Ok(serde_json::to_string_pretty(&value)?)
}
