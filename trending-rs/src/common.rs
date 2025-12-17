use std::time::Duration;

#[cfg(feature = "blocking")]
use reqwest::blocking::Client as BlockClient;
use reqwest::{
  Client as AsyncClient, Method, Proxy,
  header::{AsHeaderName, HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::errors::{ReqwestClientSnafu, Result};

pub struct TrendingClient {
  client: AsyncClient,
}

impl TrendingClient {
  pub fn new() -> Self {
    let client = AsyncClient::new();
    Self { client }
  }

  pub fn new_with_options(options: ClientOptions) -> Result<Self> {
    let mut client_builder = AsyncClient::builder();
    if let Some(timeout) = options.timeout {
      client_builder = client_builder.timeout(timeout);
    }
    if let Some(proxy) = options.proxy {
      client_builder = client_builder.proxy(proxy);
    }
    let client = client_builder
      .default_headers(options.headers)
      .build()
      .context(ReqwestClientSnafu)?;
    Ok(TrendingClient { client })
  }

  pub async fn trending_zhihu(&self) -> Result<TrendingsRes> {
    crate::zhihu::trending(&self.client).await
  }

  pub async fn trending_weibo(&self) -> Result<TrendingsRes> {
    crate::weibo::trending(&self.client).await
  }

  pub async fn trending_toutiao(&self) -> Result<TrendingsRes> {
    crate::toutiao::trending(&self.client).await
  }

  pub async fn trending_tencent(&self) -> Result<TrendingsRes> {
    crate::tencent::trending(&self.client).await
  }

  pub async fn trending_tieba(&self) -> Result<TrendingsRes> {
    crate::tieba::trending(&self.client).await
  }

  pub async fn trending_netease(&self) -> Result<TrendingsRes> {
    crate::netease::trending(&self.client).await
  }
}

#[cfg(feature = "blocking")]
pub struct BlockTrendingClient {
  client: BlockClient,
}

#[cfg(feature = "blocking")]
impl BlockTrendingClient {
  pub fn new() -> Self {
    let client = BlockClient::new();
    Self { client }
  }

  pub fn new_with_options(options: ClientOptions) -> Result<Self> {
    let mut client_builder = BlockClient::builder();
    if let Some(timeout) = options.timeout {
      client_builder = client_builder.timeout(timeout);
    }
    if let Some(proxy) = options.proxy {
      client_builder = client_builder.proxy(proxy);
    }
    let client = client_builder
      .default_headers(options.headers)
      .build()
      .context(ReqwestClientSnafu)?;
    Ok(BlockTrendingClient { client })
  }

  pub fn trending_zhihu(&self) -> Result<TrendingsRes> {
    crate::zhihu::block_trending(&self.client)
  }

  pub fn trending_weibo(&self) -> Result<TrendingsRes> {
    crate::weibo::block_trending(&self.client)
  }

  pub fn trending_toutiao(&self) -> Result<TrendingsRes> {
    crate::toutiao::blocking_trending(&self.client)
  }

  pub fn trending_tencent(&self) -> Result<TrendingsRes> {
    crate::tencent::blocking_trending(&self.client)
  }

  pub fn trending_tieba(&self) -> Result<TrendingsRes> {
    crate::tieba::blocking_trending(&self.client)
  }

  pub fn trending_netease(&self) -> Result<TrendingsRes> {
    crate::netease::blocking_trending(&self.client)
  }
}

#[derive(Debug, Clone)]
pub struct ClientOptions {
  pub headers: HeaderMap,
  pub timeout: Option<Duration>,
  pub proxy: Option<Proxy>,
}

impl ClientOptions {
  pub fn new() -> ClientOptions {
    ClientOptions {
      headers: HeaderMap::new(),
      timeout: None,
      proxy: None,
    }
  }

  pub fn with_headers(mut self, headers: HeaderMap) -> Self {
    self.headers.extend(headers);
    self
  }

  pub fn with_header(mut self, key: HeaderName, value: HeaderValue) -> Self {
    self.headers.insert(key, value);
    self
  }

  pub fn with_proxy(mut self, proxy: Proxy) -> Self {
    self.proxy = Some(proxy);
    self
  }

  pub fn with_timeout(mut self, timeout: Duration) -> Self {
    self.timeout = Some(timeout);
    self
  }

  pub fn contains_header(&self, key: impl AsHeaderName) -> bool {
    self.headers.contains_key(key)
  }
}

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
      PlatformType::Other(other) => other.as_str(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrendingsRes {
  #[serde(rename = "platform")]
  pub platform: PlatformType,

  #[serde(rename = "trendings")]
  pub trendings: Vec<TrendingRes>,
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
