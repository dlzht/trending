use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::common::{EmptyType, PlatformType, TrendingRes, TrendingsRes, not_empty_str, http_get};
use crate::errors::Result;

pub const TRENDING_ENDPOINT: &'static str =
  "https://www.toutiao.com/hot-event/hot-board/?origin=toutiao_pc";

pub async fn trending(client: &Client) -> Result<TrendingsRes> {
  http_get::<EmptyType, EmptyType, TouTiaoRes>(client, TRENDING_ENDPOINT, None, None, None)
    .await
    .map(|r| r.into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TouTiaoRes {
  #[serde(rename = "data")]
  data: Vec<TouTiaoData>,

  #[serde(rename = "fixed_top_data")]
  fixed_top_data: Vec<TouTiaoData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TouTiaoData {
  #[serde(rename = "Title")]
  title: String,

  #[serde(rename = "Url")]
  url: String,

  #[serde(rename = "HotValue")]
  hot_value: Option<String>,
}

impl From<TouTiaoData> for TrendingRes {
  fn from(value: TouTiaoData) -> Self {
    Self {
      title: value.title,
      url: value.url,
      trend: not_empty_str(value.hot_value),
    }
  }
}

impl From<TouTiaoRes> for TrendingsRes {
  fn from(value: TouTiaoRes) -> Self {
    Self {
      platform: PlatformType::Toutiao,
      trendings: value
        .data
        .into_iter()
        .chain(value.fixed_top_data.into_iter())
        .map(|r| r.into())
        .collect(),
    }
  }
}
