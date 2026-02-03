//! Notion API Reverse Engineering Library
//!
//! This library provides tools for exploring and interacting with Notion's undocumented API.

pub mod api;
pub mod models;
pub mod utils;

pub use api::{NotionClient, NotionHeaders};
pub use api::paths;
