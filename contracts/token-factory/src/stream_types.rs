use soroban_sdk::{contracttype, Address, String, Vec};
use crate::types::{Error, PaginationCursor};

/// Stream information with optional metadata
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamInfo {
    pub id: u32,
    pub creator: Address,
    pub recipient: Address,
    pub amount: i128,
    pub metadata: Option<String>,
    pub created_at: u64,
}

/// Paginated stream result
///
/// Contains a page of streams and a cursor for fetching the next page.
///
/// # Fields
/// * `streams` - Vector of stream info for this page
/// * `cursor` - Cursor for next page (None = no more results)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaginatedStreams {
    pub streams: Vec<StreamInfo>,
    pub cursor: Option<PaginationCursor>,
}

/// Validate stream metadata length (max 512 chars)
pub fn validate_metadata(metadata: &Option<String>) -> Result<(), Error> {
    if let Some(meta) = metadata {
        let len = meta.len();
        if len == 0 || len > 512 {
            return Err(Error::InvalidParameters);
        }
    }
    Ok(())
}
