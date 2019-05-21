use serde::{Serialize, Deserialize};
use serde_json;

pub type Result<T> = std::result::Result<T, String>;

/// `CollectionInfo` is a single collection info in the CommonCrawl
/// json file at https://index.commoncrawl.org/collinfo.json
#[derive(Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CollectionInfo {
    pub id: String,
    pub name: String,
    pub timegate: String,
    pub cdx_api: String,
}

impl CollectionInfo {
    /// `new` creates a new `CollectionInfo`.
    pub fn new() -> CollectionInfo {
        CollectionInfo::default()
    }

    /// `from_json_string` deserializes a new `CollectionInfo` from a json string.
    pub fn from_json_string(s: &str) -> Result<CollectionInfo> {
        serde_json::from_str(s)
            .map_err(|e| format!("{}", e))
    }

    /// `to_json_string` serializes the `CollectionInfo` into a json string.
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self)
            .map_err(|e| format!("{}", e))
    }

    /// `from_json_bytes` deserializes a new `CollectionInfo` from a json bytes.
    pub fn from_json_bytes(b: &[u8]) -> Result<CollectionInfo> {
        serde_json::from_slice(b)
            .map_err(|e| format!("{}", e))
    }

    /// `to_json_bytes` serializes the `CollectionInfo` into a json bytes.
    pub fn to_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| format!("{}", e))
    }
}

/// `CollectionsInfo` is a collection of `CollectionInfo`s.
#[derive(Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CollectionsInfo(Vec<CollectionInfo>);

impl CollectionsInfo {
    /// `new` creates a new `CollectionsInfo`.
    pub fn new() -> CollectionsInfo {
        CollectionsInfo::default()
    }

    /// `from_json_string` deserializes a new `CollectionsInfo` from a json string.
    pub fn from_json_string(s: &str) -> Result<CollectionsInfo> {
        serde_json::from_str(s)
            .map_err(|e| format!("{}", e))
    }

    /// `to_json_string` serializes the `CollectionsInfo` into a json string.
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self)
            .map_err(|e| format!("{}", e))
    }

    /// `from_json_bytes` deserializes a new `CollectionsInfo` from a json bytes.
    pub fn from_json_bytes(b: &[u8]) -> Result<CollectionsInfo> {
        serde_json::from_slice(b)
            .map_err(|e| format!("{}", e))
    }

    /// `to_json_bytes` serializes the `CollectionsInfo` into a json bytes.
    pub fn to_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| format!("{}", e))
    }
}

/// `CDXItem` is a single item returned by a CDX query.
#[derive(Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub struct CDXItem {
    pub urlkey: String,
    pub timestamp: u64,
    pub mime: String,
    pub length: u64,
    pub status: u64,
    pub filename: String,
    pub languages: String,
    pub charset: String,
    pub url: String,
    pub mime_detected: String,
    pub offset: u64,
    pub digest: String,
}

/// `CDXItems` is the collection of items returned by a CDX query.
#[derive(Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CDXItems(Vec<CDXItem>);
