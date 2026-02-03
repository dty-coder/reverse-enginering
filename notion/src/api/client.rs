use anyhow::{Context, Result};
use reqwest::{Client, RequestBuilder};
use serde::Serialize;
use tracing::{debug, trace};

use super::NotionHeaders;

/// Notion API Client for reverse engineering
#[derive(Debug, Clone)]
pub struct NotionClient {
    client: Client,
    headers: NotionHeaders,
}

impl NotionClient {
    pub fn new(headers: NotionHeaders) -> Self {
        Self {
            client: Client::builder()
                .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
                .build()
                .unwrap(),
            headers,
        }
    }

    /// Get authentication token from browser cookies
    ///
    /// Token is stored in `token_v2` cookie
    pub fn from_token(token: String) -> Self {
        Self::new(NotionHeaders::new(token))
    }

    /// Build a request with Notion-specific headers
    fn build_request(&self, mut req: RequestBuilder) -> RequestBuilder {
        req = req
            .header("Cookie", format!("token_v2={}", self.headers.token))
            .header("Content-Type", "application/json");

        if let Some(user_id) = &self.headers.user_id {
            req = req.header("x-notion-active-user-header", user_id);
        }

        req
    }

    /// Make a GET request to Notion API
    pub async fn get<T: Serialize>(&self, path: &str, query: Option<&T>) -> Result<String> {
        let url = format!("{}{}", super::NOTION_API_BASE, path);
        debug!("GET {}", url);

        let mut req = self.client.get(&url);
        req = self.build_request(req);

        if let Some(q) = query {
            req = req.query(q);
        }

        let resp = req.send().await.context("Failed to send request")?;
        let status = resp.status();
        let body = resp.text().await.context("Failed to read response")?;

        trace!("Response status: {}", status);
        trace!("Response body: {}", body);

        if !status.is_success() {
            anyhow::bail!("Request failed with status {}: {}", status, body);
        }

        Ok(body)
    }

    /// Make a POST request to Notion API
    pub async fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<String> {
        let url = format!("{}{}", super::NOTION_API_BASE, path);
        debug!("POST {}", url);

        let req = self.client.post(&url);
        let req = self.build_request(req).json(body);

        let resp = req.send().await.context("Failed to send request")?;
        let status = resp.status();
        let resp_body = resp.text().await.context("Failed to read response")?;

        trace!("Response status: {}", status);
        trace!("Response body: {}", resp_body);

        if !status.is_success() {
            anyhow::bail!("Request failed with status {}: {}", status, resp_body);
        }

        Ok(resp_body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_from_token() {
        let token = "test_token_123".to_string();
        let client = NotionClient::from_token(token.clone());

        assert_eq!(client.headers.token, token);
        assert!(client.headers.user_id.is_none());
    }

    #[test]
    fn test_client_with_user_id() {
        let headers = NotionHeaders::new("test_token".to_string())
            .with_user_id("user_123".to_string());
        let client = NotionClient::new(headers);

        assert_eq!(client.headers.token, "test_token");
        assert_eq!(client.headers.user_id, Some("user_123".to_string()));
    }

    #[test]
    fn test_notion_headers_new() {
        let headers = NotionHeaders::new("my_token".to_string());

        assert_eq!(headers.token, "my_token");
        assert!(headers.user_id.is_none());
    }

    #[test]
    fn test_notion_headers_with_user_id() {
        let headers = NotionHeaders::new("my_token".to_string())
            .with_user_id("user_456".to_string());

        assert_eq!(headers.token, "my_token");
        assert_eq!(headers.user_id, Some("user_456".to_string()));
    }

    #[test]
    fn test_notion_headers_chaining() {
        let headers = NotionHeaders::new("token".to_string())
            .with_user_id("user".to_string());

        // Verify that with_user_id consumes and returns the struct
        assert_eq!(headers.token, "token");
        assert_eq!(headers.user_id.as_deref(), Some("user"));
    }
}
