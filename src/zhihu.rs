use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::common::{EmptyType, PlatformType, TrendingRes, TrendingsRes, not_empty_str, http_get};
use crate::errors::Result;

pub const TRENDING_ENDPOINT: &'static str = "https://api.zhihu.com/topstory/hot-lists/total";

pub async fn trending(client: &Client) -> Result<TrendingsRes> {
  http_get::<EmptyType, EmptyType, ZhihuRes>(client, TRENDING_ENDPOINT, None, None, None)
    .await
    .map(|r| r.into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ZhihuRes {
  #[serde(rename = "data", skip_serializing_if = "Vec::is_empty", default)]
  data: Vec<ZhihuData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ZhihuData {
  #[serde(rename = "target")]
  target: ZhihuTarget,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ZhihuTarget {
  #[serde(rename = "title")]
  title: String,

  #[serde(rename = "url")]
  url: String,

  #[serde(rename = "detail_text", skip_serializing_if = "Option::is_none")]
  detail_text: Option<String>,
}

impl From<ZhihuData> for TrendingRes {
  fn from(value: ZhihuData) -> Self {
    Self {
      title: value.target.title,
      url: value.target.url,
      trend: not_empty_str(value.target.detail_text),
    }
  }
}

impl From<ZhihuRes> for TrendingsRes {
  fn from(value: ZhihuRes) -> Self {
    Self {
      platform: PlatformType::Zhihu,
      trendings: value.data.into_iter().map(ZhihuData::into).collect(),
    }
  }
}
