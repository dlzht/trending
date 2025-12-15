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

pub const TRENDING_ENDPOINT: &'static str =
  "https://r.inews.qq.com/gw/event/hot_ranking_list?page_size=30";

pub async fn trending(client: &AsyncClient) -> Result<TrendingsRes> {
  http_get::<EmptyType, EmptyType, TencentRes>(client, TRENDING_ENDPOINT, None, None, None)
    .await
    .map(|r| r.into())
}

#[cfg(feature = "blocking")]
pub fn blocking_trending(client: &BlockClient) -> Result<TrendingsRes> {
  block_http_get::<EmptyType, EmptyType, TencentRes>(client, TRENDING_ENDPOINT, None, None, None)
    .map(|r| r.into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TencentRes {
  #[serde(rename = "idlist", skip_serializing_if = "Vec::is_empty", default)]
  list: Vec<TencentList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TencentList {
  #[serde(rename = "newslist", skip_serializing_if = "Vec::is_empty", default)]
  news: Vec<TencentNews>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TencentNews {
  #[serde(rename = "id")]
  id: String,

  #[serde(rename = "title")]
  title: String,

  #[serde(rename = "surl")]
  url: Option<String>,
  
  #[serde(rename = "ranking")]
  ranking: Option<u32>
}

impl From<TencentNews> for TrendingRes {
  fn from(value: TencentNews) -> Self {
    Self {
      title: value.title,
      url: value.url.unwrap_or_else(String::new),
      trend: value.ranking.map(|r| r.to_string()),
    }
  }
}

impl From<TencentRes> for TrendingsRes {
  fn from(value: TencentRes) -> Self {
    Self {
      platform: PlatformType::Tencent,
      trendings: value
        .list
        .into_iter()
        .flat_map(|r| r.news.into_iter())
        .filter(|r| r.url.is_some())
        .map(|r| r.into())
        .collect(),
    }
  }
}
