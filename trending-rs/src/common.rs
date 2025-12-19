#[cfg(feature = "blocking")]
use reqwest::blocking::Client as BlockClient;
use reqwest::{Client as AsyncClient, Method, header::HeaderMap};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::errors::{ReqwestClientSnafu, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlatformType {
  #[serde(rename = "zhihu")]
  Zhihu,

  #[serde(rename = "weibo")]
  Weibo,

  #[serde(rename = "toutiao")]
  Toutiao,

  #[serde(rename = "tencent")]
  Tencent,

  #[serde(rename = "tieba")]
  Tieba,

  #[serde(rename = "netease")]
  Netease,

  #[serde(rename = "hupu")]
  Hupu,

  #[serde(untagged)]
  Other(String),
}

impl PlatformType {
  pub fn to_str(&self) -> &str {
    match self {
      PlatformType::Zhihu => "zhihu",
      PlatformType::Weibo => "weibo",
      PlatformType::Toutiao => "toutiao",
      PlatformType::Tencent => "tencent",
      PlatformType::Tieba => "tieba",
      PlatformType::Netease => "netease",
      PlatformType::Hupu => "hupu",
      PlatformType::Other(other) => other.as_str(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendingsRes {
  #[serde(rename = "platform")]
  pub platform: PlatformType,

  #[serde(rename = "trendings", skip_serializing_if = "Vec::is_empty", default)]
  pub result: Vec<TrendingRes>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendingRes {
  #[serde(rename = "title")]
  pub title: String,

  #[serde(rename = "url")]
  pub url: String,

  #[serde(rename = "trend")]
  pub trend: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchesRes {
  #[serde(rename = "platform")]
  pub platform: PlatformType,

  #[serde(rename = "searches", skip_serializing_if = "Vec::is_empty", default)]
  pub result: Vec<SearchRes>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PageParam {
  First,
  Other(u32),
}

impl From<u32> for PageParam {
  fn from(value: u32) -> Self {
    Self::Other(value)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchReq {
  #[serde(rename = "keyword")]
  pub keyword: String,

  #[serde(rename = "page")]
  pub page: Option<PageParam>,

  #[serde(rename = "size")]
  pub size: Option<u32>,
}

impl SearchReq {
  pub fn new(keyword: impl Into<String>) -> Self {
    Self {
      keyword: keyword.into(),
      page: None,
      size: None,
    }
  }

  pub fn with_page(mut self, page: impl Into<PageParam>) -> Self {
    self.page = Some(page.into());
    self
  }

  pub fn with_size(mut self, size: u32) -> Self {
    self.size = Some(size);
    self
  }
}

impl From<&str> for SearchReq {
  fn from(value: &str) -> Self {
    Self::new(value)
  }
}

impl From<(&str, u32)> for SearchReq {
  fn from(value: (&str, u32)) -> Self {
    Self::new(value.0).with_page(value.1)
  }
}

impl From<(&str, u32, u32)> for SearchReq {
  fn from(value: (&str, u32, u32)) -> Self {
    Self::new(value.0).with_page(value.1).with_size(value.2)
  }
}

impl From<String> for SearchReq {
  fn from(value: String) -> Self {
    Self::new(value)
  }
}

impl From<(String, u32)> for SearchReq {
  fn from(value: (String, u32)) -> Self {
    Self::new(value.0).with_page(value.1)
  }
}

impl From<(String, u32, u32)> for SearchReq {
  fn from(value: (String, u32, u32)) -> Self {
    Self::new(value.0).with_page(value.1).with_size(value.2)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchRes {
  #[serde(rename = "title")]
  pub title: String,

  #[serde(rename = "url")]
  pub url: String,

  #[serde(rename = "time")]
  pub time: Option<u64>,

  #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
  pub images: Option<Vec<String>>,

  #[serde(rename = "videos", skip_serializing_if = "Option::is_none")]
  pub videos: Option<Vec<String>>,

  #[serde(rename = "audios", skip_serializing_if = "Option::is_none")]
  pub audios: Option<Vec<String>>,
}

pub(crate) fn not_empty_str(text: Option<String>) -> Option<String> {
  return if let Some(s) = &text
    && !s.is_empty()
  {
    text
  } else {
    None
  };
}

pub(crate) async fn http_get<
  Q: Serialize + ?Sized,
  B: Serialize + ?Sized,
  R: for<'de> Deserialize<'de>,
>(
  client: &AsyncClient,
  url: &str,
  headers: Option<HeaderMap>,
  queries: Option<&Q>,
  json: Option<&B>,
) -> Result<R> {
  http_execute(client, Method::GET, url, headers, queries, json).await
}

pub(crate) async fn _http_post<
  Q: Serialize + ?Sized,
  B: Serialize + ?Sized,
  R: for<'de> Deserialize<'de>,
>(
  client: &AsyncClient,
  url: &str,
  headers: Option<HeaderMap>,
  queries: Option<&Q>,
  json: Option<&B>,
) -> Result<R> {
  http_execute(client, Method::POST, url, headers, queries, json).await
}

async fn http_execute<
  Q: Serialize + ?Sized,
  B: Serialize + ?Sized,
  R: for<'de> Deserialize<'de>,
>(
  client: &AsyncClient,
  method: Method,
  url: &str,
  headers: Option<HeaderMap>,
  queries: Option<&Q>,
  json: Option<&B>,
) -> Result<R> {
  let mut req = client.request(method, url);
  if let Some(headers) = headers {
    req = req.headers(headers);
  }
  if let Some(queries) = queries {
    req = req.query(queries);
  }
  if let Some(json) = json {
    req = req.json(json);
  }
  let res = req
    .send()
    .await
    .context(ReqwestClientSnafu)?
    .json::<R>()
    .await
    .context(ReqwestClientSnafu)?;
  Ok(res)
}

#[cfg(feature = "blocking")]
pub(crate) fn block_http_get<
  Q: Serialize + ?Sized,
  B: Serialize + ?Sized,
  R: for<'de> Deserialize<'de>,
>(
  client: &BlockClient,
  url: &str,
  headers: Option<HeaderMap>,
  queries: Option<&Q>,
  json: Option<&B>,
) -> Result<R> {
  block_http_execute(client, Method::GET, url, headers, queries, json)
}

#[cfg(feature = "blocking")]
pub(crate) fn _block_http_post<
  Q: Serialize + ?Sized,
  B: Serialize + ?Sized,
  R: for<'de> Deserialize<'de>,
>(
  client: &BlockClient,
  url: &str,
  headers: Option<HeaderMap>,
  queries: Option<&Q>,
  json: Option<&B>,
) -> Result<R> {
  block_http_execute(client, Method::POST, url, headers, queries, json)
}

#[cfg(feature = "blocking")]
fn block_http_execute<
  Q: Serialize + ?Sized,
  B: Serialize + ?Sized,
  R: for<'de> Deserialize<'de>,
>(
  client: &BlockClient,
  method: Method,
  url: &str,
  headers: Option<HeaderMap>,
  queries: Option<&Q>,
  json: Option<&B>,
) -> Result<R> {
  let mut req = client.request(method, url);
  if let Some(headers) = headers {
    req = req.headers(headers);
  }
  if let Some(queries) = queries {
    req = req.query(queries);
  }
  if let Some(json) = json {
    req = req.json(json);
  }
  let res = req
    .send()
    .context(ReqwestClientSnafu)?
    .json::<R>()
    .context(ReqwestClientSnafu)?;
  Ok(res)
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) struct EmptyType;
