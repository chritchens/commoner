use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
}

/// `CollectionsInfo` is a collection of `CollectionInfo`s keyed by id.
#[derive(Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CollectionsInfo(HashMap<String, CollectionInfo>);

impl CollectionsInfo {
    /// `new` creates a new `CollectionsInfo`.
    pub fn new() -> CollectionsInfo {
        CollectionsInfo(HashMap::new())
    }
}
