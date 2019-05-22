use serde::{Serialize, Deserialize};
use serde_json;

pub type Result<T> = std::result::Result<T, String>;

/// `ToJson` specifies the operations implemented by types that can be serialized into JSON.
pub trait ToJson<'a>: Serialize + Deserialize<'a> {
    /// `to_json_string` serializes the implementor into a json string.
    fn to_json_string(&self) -> Result<String> {
        serde_json::to_string(self)
            .map_err(|e| format!("{}", e))
    }
    
    /// `to_json_bytes` serializes the implementor into json bytes.
    fn to_json_bytes(&self) -> Result<Vec<u8>> {
        serde_json::to_vec(self)
            .map_err(|e| format!("{}", e))
    }
}

/// `FromJson` specifies the operations implemented by types that can be deserialized from JSON.
pub trait FromJson<'a>: Serialize + Deserialize<'a> {
    /// `from_json_string` deserializes an instance of the implementor from a json string.
    fn from_json_string(s: &'a str) -> Result<Self> {
        serde_json::from_str(s)
            .map_err(|e| format!("{}", e))
    }

    /// `from_json_bytes` deserializes an instance of the implementor from json bytes.
    fn from_json_bytes(b: &'a [u8]) -> Result<Self> {
        serde_json::from_slice(b)
            .map_err(|e| format!("{}", e))
    }
}

/// `Fetch` specifies the operations of the types that can be fetched from remote.
pub trait Fetch<'a>: Deserialize<'a> {
    /// `DEFAULT_LOCATION` is the default location of the resource to fetch.
    const DEFAULT_LOCATION: &'a str;

    /// `fetch_from_location` creates an instance of the implementor from a given remote location.
    fn fetch_from_location(l: &str) -> Result<Self>;

    /// `fetch` creates an instance of the implementor from the default remote location.
    fn fetch() -> Result<Self> {
        Self::fetch_from_location(Self::DEFAULT_LOCATION)
    }
}

/// `FetchJson` specifies the operations of the types that can be fetched as json from remote.
pub trait FetchJson<'a>: FromJson<'a> {
    /// `DEFAULT_LOCATION` is the default location of the resource to fetch.
    const DEFAULT_LOCATION: &'a str;

    /// `fetch_json_from_location` creates an instance of the implementor from a given remote location.
    /// The resource is expected to be in Json format.
    fn fetch_json_from_location(l: &str) -> Result<Self>;

    /// `fetch_json` creates an instance of the implementor from the default remote location.
    /// The resource is expected to be in Json format.
    fn fetch_json() -> Result<Self> {
        Self::fetch_json_from_location(Self::DEFAULT_LOCATION)
    }
}

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

impl<'a> ToJson<'a> for CollectionInfo {}

impl<'a> FromJson<'a> for CollectionInfo {}

/// `CollectionsInfo` is a collection of `CollectionInfo`s.
#[derive(Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CollectionsInfo(Vec<CollectionInfo>);

impl CollectionsInfo {
    /// `new` creates a new `CollectionsInfo`.
    pub fn new() -> CollectionsInfo {
        CollectionsInfo::default()
    }
}

impl<'a> ToJson<'a> for CollectionsInfo {}

impl<'a> FromJson<'a> for CollectionsInfo {}

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

impl CDXItem {
    /// `new` creates a new `CDXItem`.
    pub fn new() -> CDXItem {
        CDXItem::default()
    }
}

impl<'a> ToJson<'a> for CDXItem {}

impl<'a> FromJson<'a> for CDXItem {}

/// `CDXItems` is the collection of items returned by a CDX query.
#[derive(Default, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CDXItems(Vec<CDXItem>);

impl CDXItems {
    /// `new` creates a new `CDXItems`.
    pub fn new() -> CDXItems {
        CDXItems::default()
    }
}

impl<'a> ToJson<'a> for CDXItems {}

impl<'a> FromJson<'a> for CDXItems {}
