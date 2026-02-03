// Utility functions for Notion API reverse engineering

use std::fs;
use std::path::Path;
use anyhow::Result;

/// Save response to a file for analysis
pub fn save_response<P: AsRef<Path>>(path: P, data: &str) -> Result<()> {
    fs::write(path, data)?;
    Ok(())
}

/// Pretty print JSON response
pub fn pretty_print_json(json: &str) -> Result<String> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    Ok(serde_json::to_string_pretty(&value)?)
}

/// Extract page ID from a Notion URL
///
/// Supports two formats:
/// 1. https://www.notion.so/1234567890abcdef1234567890abcdef (direct ID)
/// 2. https://www.notion.so/username/Page-Title-1234567890abcdef (slug format)
///
/// Extracts: 1234567890abcdef
pub fn extract_page_id_from_url(url: &str) -> Option<String> {
    // First try: Extract from path segments (direct ID format)
    if let Some(path) = url.strip_prefix("https://www.notion.so/") {
        // Remove query parameters
        let path = path.split('?').next().unwrap_or(path);
        let segment = path.trim_end_matches('/');

        // Check if the path segment itself is a valid Notion ID
        if is_valid_notion_id(segment) {
            return Some(segment.to_string());
        }

        // Second try: Extract from slug format (Page-Title-ID)
        if let Some(last_part) = segment.rsplit('-').next() {
            if is_valid_notion_id(last_part) {
                return Some(last_part.to_string());
            }
        }
    }

    None
}

/// Validate Notion ID format (32 character hex string)
pub fn is_valid_notion_id(id: &str) -> bool {
    id.len() == 32 && id.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_page_id() {
        let url = "https://www.notion.so/username/Test-Page-1234567890abcdef1234567890abcdef";
        assert_eq!(
            extract_page_id_from_url(url),
            Some("1234567890abcdef1234567890abcdef".to_string())
        );
    }

    #[test]
    fn test_extract_page_id_with_trailing_slash() {
        let url = "https://www.notion.so/username/Test-Page-1234567890abcdef1234567890abcdef/";
        assert_eq!(
            extract_page_id_from_url(url),
            Some("1234567890abcdef1234567890abcdef".to_string())
        );
    }

    #[test]
    fn test_extract_real_notion_url() {
        // Real Notion URL format
        let url = "https://www.notion.so/287502506d2c800f9c00c9f8a5e285e3?v=287502506d2c814eb0c6000cbb508a4e";
        assert_eq!(
            extract_page_id_from_url(url),
            Some("287502506d2c800f9c00c9f8a5e285e3".to_string())
        );
    }

    #[test]
    fn test_extract_page_id_no_match() {
        let url = "https://www.notion.so/username/test";
        assert_eq!(extract_page_id_from_url(url), None);
    }

    #[test]
    fn test_valid_notion_id() {
        assert!(is_valid_notion_id("1234567890abcdef1234567890abcdef"));
        assert!(is_valid_notion_id("00000000000000000000000000000000"));
        assert!(is_valid_notion_id("ffffffffffffffffffffffffffffffff"));
        assert!(is_valid_notion_id("ABCDEF1234567890ABCDEF1234567890"));
    }

    #[test]
    fn test_invalid_notion_id() {
        assert!(!is_valid_notion_id("invalid"));
        assert!(!is_valid_notion_id("1234567890abcdef1234567890abcde")); // 31 chars
        assert!(!is_valid_notion_id("1234567890abcdef1234567890abcdefg")); // 33 chars
        assert!(!is_valid_notion_id("1234567890abcdef1234567890abcd")); // 30 chars
        assert!(!is_valid_notion_id("")); // empty
        assert!(!is_valid_notion_id("g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g-g")); // invalid chars
    }

    #[test]
    fn test_pretty_print_json() {
        let json = r#"{"name":"test","value":123}"#;
        let result = pretty_print_json(json).unwrap();
        assert!(result.contains("\n"));
        assert!(result.contains("\"name\""));
        assert!(result.contains("\"test\""));
    }

    #[test]
    fn test_pretty_print_json_invalid() {
        let json = "not valid json";
        assert!(pretty_print_json(json).is_err());
    }
}
