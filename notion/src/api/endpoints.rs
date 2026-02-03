/// Known Notion API endpoints for reverse engineering
///
/// Notion uses a REST-ish API with various endpoints for different operations.
/// Many endpoints are undocumented and may change without notice.

pub mod paths {
    // Authentication & User
    pub const GET_USER: &str = "/v3/getUser";
    pub const GET_USERS: &str = "/v3/getUsers";
    pub const GET_USER_EMAIL: &str = "/v3/getUserEmail";

    // Spaces & Workspaces
    pub const GET_SPACES: &str = "/v3/getSpaces";
    pub const CREATE_SPACE: &str = "/v3/createSpace";

    // Pages & Blocks
    pub const LOAD_PAGE_CHUNK: &str = "/v3/loadPageChunk";
    pub const GET_BLOCK: &str = "/v3/getBlock";
    pub const GET_BLOCKS: &str = "/v3/getBlocks";
    pub const GET_RECORD_VALUES: &str = "/v3/getRecordValues";
    pub const QUERY_COLLECTION: &str = "/v3/queryCollection";
    pub const QUERY_COLLECTION_VIEW: &str = "/v3/queryCollectionView";

    // Editing
    pub const SUBMIT_TRANSACTION: &str = "/v3/submitTransaction";
    pub const UPDATE_BLOCK: &str = "/v3/updateBlock";

    // Search
    pub const SEARCH: &str = "/v3/search";
    pub const SEARCH_BLOCKS: &str = "/v3/searchBlocks";

    // Databases (Collections)
    pub const GET_COLLECTION: &str = "/v3/getCollection";
    pub const GET_COLLECTION_VIEW: &str = "/v3/getCollectionView";

    // Export
    pub const EXPORT_PAGE: &str = "/v3/exportPage";
    pub const EXPORT_MATH: &str = "/v3/exportMath";

    // Upload
    pub const UPLOAD_FILE: &str = "/v3/uploadFile";
    pub const GET_SIGNED_URLS: &str = "/v3/getSignedUrls";

    // Analytics & Telemetry
    pub const SEND_EVENT: &str = "/v3/sendEvent";
}

/// Request body structures for various endpoints

#[derive(Debug, serde::Serialize)]
pub struct LoadPageChunkRequest {
    pub page_id: String,
    pub chunk_number: u32,
    pub cur_cursor: Option<Cursor>,
    pub vertical_columns: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
pub struct Cursor {
    pub stack: Vec<serde_json::Value>,
}

#[derive(Debug, serde::Serialize)]
pub struct SearchRequest {
    pub query: String,
    pub ancestor_id: Option<String>,
    pub space_id: Option<String>,
    pub limit: Option<u32>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetBlockRequest {
    pub block_id: String,
}

#[derive(Debug, serde::Serialize)]
pub struct GetBlocksRequest {
    pub block_ids: Vec<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct SubmitTransactionRequest {
    pub request_id: String,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, serde::Serialize)]
pub struct Transaction {
    pub id: String,
    pub operations: Vec<Operation>,
}

#[derive(Debug, serde::Serialize)]
pub struct Operation {
    pub id: String,
    pub table: String,
    pub path: Vec<String>,
    pub command: String,
    pub args: serde_json::Value,
}
