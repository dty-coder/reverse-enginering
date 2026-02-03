# Notion API Reverse Engineering

A Rust library and toolset for exploring and interacting with Notion's undocumented API.

## Overview

This project provides tools to reverse engineer and interact with Notion's internal API. It includes:

- **HTTP Client**: Async client for making authenticated requests to Notion's API
- **API Endpoints**: Definitions of known API endpoints and request structures
- **Data Models**: Serde-compatible models for API responses
- **Utilities**: Helper functions for working with Notion URLs and IDs

## Project Structure

```
notion/
├── Cargo.toml
├── src/
│   ├── main.rs       # Binary entry point
│   ├── lib.rs        # Library exports
│   ├── api/
│   │   ├── mod.rs    # API module with constants
│   │   ├── client.rs # HTTP client
│   │   └── endpoints.rs # API endpoints and request types
│   ├── models/mod.rs # Response data models
│   └── utils/mod.rs  # Utility functions
└── examples/
    ├── basic_usage.rs  # Fetch user info and spaces
    └── advanced.rs     # Load pages and search
```

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd notion/notion

# Build
cargo build --release
```

## Getting Your Notion Token

To use this tool, you need your Notion authentication token:

1. Open Notion in your browser (https://notion.so)
2. Open Developer Tools:
   - **Chrome/Edge**: `F12` or `Ctrl+Shift+I` (Windows) / `Cmd+Option+I` (Mac)
   - **Firefox**: `F12`
3. Go to the **Application** tab
4. Expand **Cookies** → `https://www.notion.so`
5. Find and copy the `token_v2` cookie value

## Usage

### Basic Example

```bash
cargo run --example basic_usage -- YOUR_TOKEN
```

Or set as environment variable:

```bash
export NOTION_TOKEN="your_token_here"
cargo run --example basic_usage
```

### Advanced Example

Load a specific Notion page:

```bash
cargo run --example advanced -- YOUR_TOKEN PAGE_ID
```

Example with real page:
```bash
cargo run --example advanced -- YOUR_TOKEN 287502506d2c800f9c00c9f8a5e285e3
```

### Library Usage

```rust
use notion_re::api::{NotionClient, paths};
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = NotionClient::from_token("your_token".to_string());

    // Fetch user info
    let response = client.post(paths::GET_USER, &json!({})).await?;
    println!("{}", response);

    Ok(())
}
```

## API Endpoints

The following endpoints are currently supported:

### User & Authentication
- `GET_USER` - Get user information
- `GET_USERS` - Get multiple users
- `GET_USER_EMAIL` - Get user email

### Spaces & Workspaces
- `GET_SPACES` - Get all spaces/workspaces
- `CREATE_SPACE` - Create a new space

### Pages & Blocks
- `LOAD_PAGE_CHUNK` - Load page content in chunks
- `GET_BLOCK` - Get a single block
- `GET_BLOCKS` - Get multiple blocks
- `GET_RECORD_VALUES` - Get record values
- `QUERY_COLLECTION` - Query a database
- `QUERY_COLLECTION_VIEW` - Query a database view

### Editing
- `SUBMIT_TRANSACTION` - Submit changes
- `UPDATE_BLOCK` - Update a block

### Search
- `SEARCH` - Search content
- `SEARCH_BLOCKS` - Search blocks

### Export & Upload
- `EXPORT_PAGE` - Export a page
- `UPLOAD_FILE` - Upload files
- `GET_SIGNED_URLS` - Get signed URLs for uploads

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_extract_page_id
```

## Known API Response Formats

### LoadPageChunk Response
```json
{
  "recordMap": {
    "block": { ... },
    "space": { ... },
    "user": { ... }
  },
  "cursor": { ... }
}
```

### Block Types
- Page, Text, Header, SubHeader, SubSubHeader
- BulletList, NumberedList, Toggle, Quote
- Code, Image, Video, File, PDF, Embed
- Table, Callout, Divider, Equation
- And many more...

## Notes

- This tool uses the **unofficial** Notion API
- Endpoints may change without notice
- Use responsibly and don't abuse rate limits
- The `token_v2` cookie is your session credential - keep it private

## License

See LICENSE file for details.

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## Disclaimer

This tool is for educational and research purposes only. Respect Notion's Terms of Service and use at your own risk.
