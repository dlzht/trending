use std::{str::FromStr, time::Duration};

use reqwest::{
  Proxy,
  header::{HeaderName, HeaderValue},
};
use snafu::ResultExt;
use trending::{
  common::{
    ClientOptions as RClientOptions, TrendingClient as RAsyncClient, TrendingRes as RTrendingRes,
    TrendingsRes as RTrendingsRes,
  },
  errors::{
    ReqwestClientSnafu, ReqwestHeaderNameSnafu, ReqwestHeaderValueSnafu,
    TrendingError as RTrendingError,
  },
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct TrendingError(RTrendingError);

impl From<RTrendingError> for TrendingError {
  fn from(err: RTrendingError) -> Self {
    Self(err)
  }
}

pub type Result<T> = std::result::Result<T, TrendingError>;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ClientOptions {
  options: RClientOptions,
}

#[wasm_bindgen]
impl ClientOptions {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Self {
      options: RClientOptions::new(),
    }
  }

  pub fn with_header(&mut self, name: &str, value: &str) -> Result<()> {
    let name = HeaderName::from_str(name).context(ReqwestHeaderNameSnafu {
      name: name.to_string(),
    })?;
    let value = HeaderValue::from_str(value).context(ReqwestHeaderValueSnafu {
      value: value.to_string(),
    })?;
    self.options.headers.insert(name, value);
    Ok(())
  }

  pub fn with_proxy(&mut self, proxy: &str) -> Result<()> {
    let proxy = Proxy::all(proxy).context(ReqwestClientSnafu)?;
    self.options.proxy = Some(proxy);
    Ok(())
  }

  pub fn with_millis_timeout(&mut self, millis: u64) {
    let timeout = Duration::from_millis(millis);
    self.options.timeout = Some(timeout);
  }

  pub fn debug_print(&self) {
    println!("{:?}", self);
  }
}

#[wasm_bindgen]
pub struct TrendingClient {
  client: RAsyncClient,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TrendingRes {
  title: String,

  url: String,

  trend: Option<String>,
}

#[wasm_bindgen]
impl TrendingRes {
  #[wasm_bindgen(getter)]
  pub fn get_title(&self) -> String {
    self.title.clone()
  }
  
  #[wasm_bindgen(getter)]
  pub fn get_url(&self) -> String {
    self.url.clone()
  }
  
  #[wasm_bindgen(getter)]
  pub fn get_trend(&self) -> Option<String> {
    self.trend.clone()
  }
}

impl From<RTrendingRes> for TrendingRes {
  fn from(value: RTrendingRes) -> Self {
    Self {
      title: value.title,
      url: value.url,
      trend: value.trend,
    }
  }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct TrendingsRes {
  platform: String,

  trendings: Vec<TrendingRes>,
}

#[wasm_bindgen]
impl TrendingsRes {
  #[wasm_bindgen(getter)]
  pub fn get_platform(&self) -> String {
    self.platform.clone()
  }
  
  #[wasm_bindgen(getter)]
  pub fn get_trendings(&self) -> Vec<TrendingRes> {
    self.trendings.clone()
  }
}

impl From<RTrendingsRes> for TrendingsRes {
  fn from(value: RTrendingsRes) -> Self {
    let trendings = value.trendings.into_iter().map(|r| r.into()).collect();
    Self {
      platform: value.platform.to_str().to_string(),
      trendings,
    }
  }
}

#[wasm_bindgen]
impl TrendingClient {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    TrendingClient {
      client: RAsyncClient::new(),
    }
  }
  
  #[wasm_bindgen(constructor)]
  pub fn new_with_options(options: ClientOptions) -> Result<Self> {
    Ok(TrendingClient { client: RAsyncClient::new_with_options(options.options)? })
  }

  pub async fn trending_zhihu(&self) -> Result<TrendingsRes> {
    let res = self.client.trending_zhihu().await?;
    Ok(res.into())
  }

  pub async fn trending_weibo(&self) -> Result<TrendingsRes> {
    let res = self.client.trending_weibo().await?;
    Ok(res.into())
  }

  pub async fn trending_toutiao(&self) -> Result<TrendingsRes> {
    let res = self.client.trending_toutiao().await?;
    Ok(res.into())
  }
}