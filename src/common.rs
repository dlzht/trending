use std::time::Duration;

use reqwest::{
  Client, Method, Proxy,
  header::{AsHeaderName, HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::errors::{ReqwestClientSnafu, Result};

pub struct TrendingClient {
  client: Client,
}

impl TrendingClient {
  pub async fn trending_zhihu(&self) -> Result<TrendingsRes> {
    crate::zhihu::trending(&self.client).await
  }

  pub async fn trending_weibo(&self) -> Result<TrendingsRes> {
    crate::weibo::trending(&self.client).await
  }

  pub async fn trending_toutiao(&self) -> Result<TrendingsRes> {
    crate::toutiao::trending(&self.client).await
  }
}

pub struct ClientOptions {
  headers: HeaderMap,
  timeout: Option<Duration>,
  proxy: Option<Proxy>,
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

  pub fn build_client(self) -> Result<TrendingClient> {
    let mut client_builder = Client::builder();
    if let Some(timeout) = self.timeout {
      client_builder = client_builder.timeout(timeout);
    }
    if let Some(proxy) = self.proxy {
      client_builder = client_builder.proxy(proxy);
    }
    let client = client_builder
      .default_headers(self.headers)
      .build()
      .context(ReqwestClientSnafu)?;
    Ok(TrendingClient { client })
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PlatformType {
  #[serde(rename = "zhihu")]
  Zhihu,

  #[serde(rename = "weibo")]
  Weibo,

  #[serde(rename = "Toutiao")]
  Toutiao,

  #[serde(untagged)]
  Other(String),
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
  client: &Client,
  url: &str,
  headers: Option<HeaderMap>,
  queries: Option<&Q>,
  json: Option<&B>,
) -> Result<R> {
  http_execute(client, Method::GET, url, headers, queries, json).await
}

pub(crate) async fn http_post<
  Q: Serialize + ?Sized,
  B: Serialize + ?Sized,
  R: for<'de> Deserialize<'de>,
>(
  client: &Client,
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
  client: &Client,
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub(crate) struct EmptyType;
