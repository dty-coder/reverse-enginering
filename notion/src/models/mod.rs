// Data models for Notion API responses

use serde::{Deserialize, Serialize};

/// Block type in Notion
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockType {
    Page,
    Text,
    Header,
    SubHeader,
    SubSubHeader,
    BulletList,
    NumberedList,
    Toggle,
    Quote,
    Divider,
    Callout,
    Todo,
    Code,
    Bookmark,
    Image,
    Video,
    File,
    PDF,
    Embed,
    Figure,
    Table,
    TableRow,
    TableOfContents,
    Breadcrumb,
    Column,
    ColumnList,
    Mention,
    Equation,
    ToggleBlock,
    Synapse,
    Collection,
    CollectionView,
    CollectionViewPage,
}

/// A Notion block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: String,
    pub version: u32,
    #[serde(rename = "type")]
    pub block_type: BlockType,
    pub properties: BlockProperties,
    pub content: Option<Vec<String>>,
    pub parent_id: Option<String>,
    pub permission_ids: Option<Vec<String>>,
    pub created_time: u64,
    pub last_edited_time: u64,
    pub alive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProperties {
    pub title: Option<Vec<RichText>>,
    pub description: Option<Vec<RichText>>,
    pub language: Option<String>,
    pub code: Option<String>,
    pub source: Option<Vec<String>>,
    pub caption: Option<Vec<RichText>>,
}

/// Rich text format used throughout Notion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RichText {
    #[serde(rename = "type")]
    pub text_type: Option<String>,
    pub text: TextContent,
    pub annotations: Option<Annotations>,
    pub plain_text: Option<String>,
    pub href: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    pub content: String,
    pub link: Option<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

/// User object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email: Option<String>,
    pub profile_photo: Option<String>,
    pub role: Option<String>,
}

/// Space (workspace) object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Space {
    pub id: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub domain: Option<String>,
}

/// Page load chunk response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadPageChunkResponse {
    pub record_map: RecordMap,
    pub cursor: Option<Cursor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordMap {
    pub block: Option<Vec<Block>>,
    pub space: Option<Vec<Space>>,
    pub user: Option<Vec<User>>,
    pub notion_user: Option<Vec<User>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    pub stack: Vec<serde_json::Value>,
}

/// Search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub results: Vec<SearchResult>,
    pub total: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub highlight: Option<String>,
    pub snapshot: Option<String>,
    pub block_id: Option<String>,
    pub parent_id: Option<String>,
}
