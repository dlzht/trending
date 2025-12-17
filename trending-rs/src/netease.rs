use reqwest::Client as AsyncClient;
#[cfg(feature = "blocking")]
use reqwest::blocking::Client as BlockClient;
use serde::{Deserialize, Serialize};

#[cfg(feature = "blocking")]
use crate::common::block_http_get;
use crate::{
  common::{EmptyType, PlatformType, TrendingRes, TrendingsRes, http_get},
  errors::Result,
};

pub const TRENDING_ENDPOINT: &'static str = "https://m.163.com/fe/api/hot/news/flow";

pub async fn trending(client: &AsyncClient) -> Result<TrendingsRes> {
  http_get::<EmptyType, EmptyType, NeteaseRes>(client, TRENDING_ENDPOINT, None, None, None)
    .await
    .map(|r| r.into())
}

#[cfg(feature = "blocking")]
pub fn blocking_trending(client: &BlockClient) -> Result<TrendingsRes> {
  block_http_get::<EmptyType, EmptyType, NeteaseRes>(client, TRENDING_ENDPOINT, None, None, None)
    .map(|r| r.into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NeteaseRes {
  #[serde(rename = "data")]
  data: NeteaseData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NeteaseData {
  #[serde(rename = "list")]
  list: Vec<NeteaseNews>
} 

#[derive(Serialize, Deserialize, Debug, Clone)]
struct NeteaseNews {
  #[serde(rename = "title")]
  title: String,

  #[serde(rename = "url")]
  url: String,
}

impl From<NeteaseNews> for TrendingRes {
  fn from(value: NeteaseNews) -> Self {
    Self {
      title: value.title,
      url: value.url,
      trend: None,
    }
  }
}

impl From<NeteaseRes> for TrendingsRes {
  fn from(value: NeteaseRes) -> Self {
    Self {
      platform: PlatformType::Netease,
      trendings: value.data.list.into_iter().map(|r| r.into()).collect()
    }
  }
}
