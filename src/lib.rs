use serde::{Serialize, Deserialize};
use serde_json;
use reqwest;
//use reqwest::{self, Method, Url, Client, ClientBuilder, Request, RequestBuilder, StatusCode, header::{self, HeaderMap, HeaderValue}};
use std::fmt;

pub type Result<T> = std::result::Result<T, String>;

/// `CDX_HOST` is the host for accessing cdx index data.
pub const CDX_HOST: &str = "index.commoncrawl.org/";
/// `WARC_HOST` is the host for accessing WARC data.
pub const WARC_HOST: &str = "commoncrawl.s3.amazonaws.com/";

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
pub enum HTTPUrl {
    CDX { path: String },
    WARC { path: String },
}

impl HTTPUrl {
    /// `to_string` returns the `HTTPUrl` string.
    pub fn to_string(&self) -> String {
       format!("{}", self)
    }

    /// `from_string` creates a `HTTPUrl` from a string.
    pub fn from_string(s: &str) -> Result<HTTPUrl> {
        let url = reqwest::Url::parse(s)
            .map_err(|e| format!("{}", e))?;

        match url.host_str() {
            Some(CDX_HOST) => Ok(HTTPUrl::CDX { path: url.path().into() }),
            Some(WARC_HOST) => Ok(HTTPUrl::WARC { path: url.path().into() }),
            _ => Err("invalid domain".into())
        }
    }
}

impl Default for HTTPUrl {
    fn default() -> HTTPUrl {
        HTTPUrl::CDX { path: String::new() }
    }
}

impl fmt::Display for HTTPUrl {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           HTTPUrl::CDX { path } => write!(f, "{}{}", CDX_HOST, path),
           HTTPUrl::WARC { path } => write!(f, "{}{}", WARC_HOST, path),
       }
   }
}

/// `HTTPCharset` is the set of charsets used by `HTTPContentType`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum HTTPCharset {
    UTF8,
    UTF16,
}

impl HTTPCharset {
    /// `to_string` returns the `HTTPCharset` string.
    pub fn to_string(self) -> String {
       format!("{}", self)
    }

    /// `from_string` creates a `HTTPCharset` from a string.
    pub fn from_string(s: &str) -> Result<HTTPCharset> {
        match s {
            "utf-8" => Ok(HTTPCharset::UTF8),
            "utf-16" => Ok(HTTPCharset::UTF16),
            _ => Err("invalid charset".into())
        }
    }
}

impl Default for HTTPCharset {
    fn default() -> HTTPCharset {
        HTTPCharset::UTF8
    }
}

impl fmt::Display for HTTPCharset {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           HTTPCharset::UTF8 => write!(f, "utf-8"),
           HTTPCharset::UTF16 => write!(f, "utf-16"),
       }
   }
}

/// `HTTPContentType` is the set of content-types used by `HTTPFetcher`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum HTTPContentType {
    JSON,
    TEXT { charset: HTTPCharset },
}

impl HTTPContentType {
    /// `to_string` returns the `HTTPContentType` string.
    pub fn to_string(self) -> String {
       format!("{}", self)
    }

    /// `from_string` creates a `HTTPContentType` from a string.
    pub fn from_string(s: &str) -> Result<HTTPContentType> {
        match s {
            "application/json" => Ok(HTTPContentType::JSON ),
            "text/plain; charset=utf-8" => Ok(HTTPContentType::TEXT { charset: HTTPCharset::UTF8 } ),
            "text/plain; charset=utf-16" => Ok(HTTPContentType::TEXT { charset: HTTPCharset::UTF16 } ),
            _ => Err("invalid content-type".into())
        }
    }
}

impl Default for HTTPContentType {
    fn default() -> HTTPContentType {
        HTTPContentType::TEXT { charset: HTTPCharset::default() }
    }
}

impl fmt::Display for HTTPContentType {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           HTTPContentType::JSON => write!(f, "application/json"),
           HTTPContentType::TEXT { charset } => {
               write!(f, "text/plain; charset={}", charset)
           },
       }
   }
}

/*
/// `HTTPFetcher` is used to fetch a remote http(s) resource.
pub struct HTTPFetcher {
    pub url: Url,
    pub content_type: String,
}

impl HTTPFetcher {
    /// `new` creates a new `HTTPFetcher`.
    pub fn new() -> HTTPFetcher {
        HTTPFetcher {
            url: Url::parse(""),
            content_type: String::new(),
        }
    }

    /// `add_host` adds the `HTTFetcher` request host.
    pub fn add_host(mut self, host: &str) -> Result<HTTPFetcher> {
        self.host = host.to_string();
        Ok(self)
    }

    /// `add_content_type` adds the `HTTFetcher` request content-type.
    pub fn add_content_type(mut self, content_type: &str) -> Result<HTTPFetcher> {
        self.content_type = content_type.to_string();
        Ok(self)
    }

    /// `run` runs the `HTTPFetcher`.
    pub fn run(self) -> Result<Vec<u8>> {
        if self.uri.is_empty() {
            return Err("missing uri".to_string());
        }

        let res = Request::new(Method::GET, self.uri.into())
            .builder()
            .header(ACCEPT, self.content_type.into())
            .send()
            .map_err(|e| format!("{}", e))?;

        if res.status() != StatusCode::OK {
            return Err("status code: {}", resp.status());
        }

        let mut contents = Vec::new();
        res.copy_to(contents)
            .map_err(|e| format!("{}", e))?;

        Ok(contents)
    }
}
*/

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
