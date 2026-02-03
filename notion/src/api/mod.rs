mod client;
pub mod endpoints;

pub use client::NotionClient;
pub use endpoints::*;

/// Notion API base URL
pub const NOTION_API_BASE: &str = "https://www.notion.so/api";
pub const NOTION_WWW_BASE: &str = "https://www.notion.so";

/// Common headers used by Notion API
#[derive(Debug, Clone)]
pub struct NotionHeaders {
    pub token: String,
    pub user_id: Option<String>,
}

impl NotionHeaders {
    pub fn new(token: String) -> Self {
        Self {
            token,
            user_id: None,
        }
    }

    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }
}
