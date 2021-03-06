use serde::{Serialize, Deserialize};
use serde_json;
use reqwest::{self, Client, StatusCode, header::{ACCEPT, HeaderValue}};
use std::fmt;

pub type Result<T> = std::result::Result<T, String>;

/// `CDX_HOST` is the host for accessing cdx index data.
pub const CDX_HOST: &str = "index.commoncrawl.org";
/// `WARC_HOST` is the host for accessing WARC data.
pub const WARC_HOST: &str = "commoncrawl.s3.amazonaws.com";

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

/// `Url` is the url type used by the library.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Url {
    CDX { path: String },
    WARC { path: String },
}

impl Url {
    /// `to_string` returns the `Url` string.
    pub fn to_string(&self) -> String {
       format!("{}", self)
    }

    /// `from_string` creates a `Url` from a string.
    pub fn from_string(s: &str) -> Result<Url> {
        let url = reqwest::Url::parse(s)
            .map_err(|e| format!("{}", e))?;

        match url.host_str() {
            Some(CDX_HOST) => Ok(Url::CDX { path: url.path().into() }),
            Some(WARC_HOST) => Ok(Url::WARC { path: url.path().into() }),
            _ => Err("invalid domain".into())
        }
    }
}

impl Default for Url {
    fn default() -> Url {
        Url::CDX { path: String::new() }
    }
}

impl fmt::Display for Url {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Url::CDX { path } => write!(f, "https://{}/{}", CDX_HOST, path),
           Url::WARC { path } => write!(f, "https://{}/{}", WARC_HOST, path),
       }
   }
}

/// `Charset` is the set of charsets used by `ContentType`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Charset {
    UTF8,
    UTF16,
}

impl Charset {
    /// `to_string` returns the `Charset` string.
    pub fn to_string(self) -> String {
       format!("{}", self)
    }

    /// `from_string` creates a `Charset` from a string.
    pub fn from_string(s: &str) -> Result<Charset> {
        match s {
            "utf-8" => Ok(Charset::UTF8),
            "utf-16" => Ok(Charset::UTF16),
            _ => Err("invalid charset".into())
        }
    }
}

impl Default for Charset {
    fn default() -> Charset {
        Charset::UTF8
    }
}

impl fmt::Display for Charset {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Charset::UTF8 => write!(f, "utf-8"),
           Charset::UTF16 => write!(f, "utf-16"),
       }
   }
}

/// `ContentType` is the set of content-types used by `Fetcher`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum ContentType {
    JSON,
    TEXT { charset: Charset },
}

impl ContentType {
    /// `to_string` returns the `ContentType` string.
    pub fn to_string(self) -> String {
       format!("{}", self)
    }

    /// `from_string` creates a `ContentType` from a string.
    pub fn from_string(s: &str) -> Result<ContentType> {
        match s {
            "application/json" => Ok(ContentType::JSON ),
            "text/plain; charset=utf-8" => Ok(ContentType::TEXT { charset: Charset::UTF8 } ),
            "text/plain; charset=utf-16" => Ok(ContentType::TEXT { charset: Charset::UTF16 } ),
            _ => Err("invalid content-type".into())
        }
    }
}

impl Default for ContentType {
    fn default() -> ContentType {
        ContentType::JSON
    }
}

impl fmt::Display for ContentType {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           ContentType::JSON => write!(f, "application/json"),
           ContentType::TEXT { charset } => {
               write!(f, "text/plain; charset={}", charset)
           },
       }
   }
}

/// `Fetcher` is used to fetch a remote http(s) resource.
#[derive(Clone, Default, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Fetcher {
    pub url: Url,
    pub content_type: ContentType,
}

impl Fetcher {
    /// `new` creates a new `Fetcher`.
    pub fn new() -> Fetcher {
        Fetcher::default()
    }

    /// `json_fetcher` creates a new json content-type `Fetcher`.
    pub fn json_fetcher(url: Url) -> Fetcher {
        Fetcher {
            url,
            content_type: ContentType::default(),
        }
    }

    /// `text_fetcher` creates a new text content-type `Fetcher`.
    pub fn text_fetcher(url: Url, charset: Charset) -> Fetcher {
        Fetcher {
            url,
            content_type: ContentType::TEXT { charset },
        }
    }

    /// `exec` execs the `Fetcher`.
    pub fn exec(self) -> Result<Vec<u8>> {
        let content_type = HeaderValue::from_str(&self.content_type.to_string())
            .map_err(|e| format!("{}", e))?;

        let req_builder = Client::new().get(&self.url.to_string());

        let mut res = req_builder
            .header(ACCEPT, content_type)
            .send()
            .map_err(|e| format!("{}", e))?;

        if res.status() != StatusCode::OK {
            return Err(format!("status code: {}", res.status()));
        }

        let mut contents = Vec::new();
        res.copy_to(&mut contents)
            .map_err(|e| format!("{}", e))?;

        Ok(contents)
    }
}

/// `CDXQuerier` is used to query the CommonCrawl Index CDX API.
#[derive(Clone, Default, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct CDXQuerier {
    pub path: String,
    pub from: u64,
    pub to: u64,
    pub limit: u64,
    pub sort: i64,
    pub filter: String,
    pub field: Option<String>,
    pub page: u64,
    pub page_size: u64,
    pub show_num_pages: bool,
    pub show_paged_index: bool,
}

impl CDXQuerier {
    /// `new` creates a new CDXQuerier.
    pub fn new() -> CDXQuerier {
        CDXQuerier::default()
    }

    /// `set_path` sets the path of the collection index.
    pub fn set_path(_path: &str) -> Result<CDXQuerier> {
        unreachable!()
    }

    /// `set_from` sets the from timestamp in the date/time range of the query.
    /// The value has to have less than 14 digits and will be padded to the
    /// lower bound.
    pub fn set_from(_from: u64) -> Result<CDXQuerier> {
        unreachable!()
    }

    /// `set_to` sets the to timestamp in the date/time range of the query.
    /// The value has to have less than 14 digits and will be padded to the
    /// upper bound.
    pub fn set_to(_to: u64) -> Result<CDXQuerier> {
        unreachable!()
    }

    /// `set_limit` sets the limit to the number of returned items from the query.
    pub fn set_limit(_limit: u64) -> Result<CDXQuerier> {
        unreachable!()
    }

    /// `set_sort` sets the sorting method in the query.
    pub fn set_sort(_sort: i64) -> Result<CDXQuerier> {
        unreachable!()
    }

    /// `set_filter` sets the filtering method in the query.
    pub fn set_filter(_filter: &str) -> Result<CDXQuerier> {
        unreachable!()
    }

    /// `set_field` sets the field to be returned if only one is required.
    pub fn set_field(_field: &str) -> Result<CDXQuerier> {
        unreachable!()
    }

    /// `set_page` sets the page to be returned by the query.
    pub fn set_page(_page: u64) -> Result<CDXQuerier> {
        unreachable!()
    }

    /// `set_page_size` sets the maximum size per page.
    pub fn set_page_size(_page_size: u64) -> Result<CDXQuerier> {
        unreachable!()
    }

    /*
    /// `set_show_num_pages` sets if the query should return the number of pages.
    pub fn set_show_num_pages(_toggle: bool) -> Result<CDXQuerier> {
        unreachable!()
    }

    /// `set_show_paged_index` sets if the query should return the
    /// secondary index data instead of the CDX data.
    pub fn set_show_paged_index(_toggle: bool) -> Result<CDXQuerier> {
        unreachable!()
    }
    */

    /// `exec` execs the `CDXQuerier`.
    pub fn exec(self) -> Result<CDXItems> {
        unreachable!()
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
    /// `PATH` is the path of the remote `CollectionsInfo`.
    pub const PATH: &'static str = "collinfo.json";

    /// `new` creates a new `CollectionsInfo`.
    pub fn new() -> CollectionsInfo {
        CollectionsInfo::default()
    }

    /// `url` returns the `CollectionsInfo` url.
    pub fn url() -> Url {
        Url::CDX { path: CollectionsInfo::PATH.into() }
    }

    /// `fetch` fetches `CollectionsInfo` from remote.
    pub fn fetch() -> Result<CollectionsInfo> {
        let url = CollectionsInfo::url();

        let fetcher = Fetcher::json_fetcher(url);

        let contents = fetcher.exec()?;

        CollectionsInfo::from_json_bytes(&contents)
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

    /// `fetch` fetches `CDXItems` from remote.
    pub fn fetch(path: &str) -> Result<CDXItems> {
        let path = if path.chars().nth(0) == Some('/') {
            let mut p = String::from(path);
            p.remove(0);
            p
        } else {
            String::from(path)
        };

        let url = Url::WARC { path };

        let fetcher = Fetcher::json_fetcher(url);

        let contents = fetcher.exec()?;

        CDXItems::from_json_bytes(&contents)
    }
}

impl<'a> ToJson<'a> for CDXItems {}

impl<'a> FromJson<'a> for CDXItems {}
