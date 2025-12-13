use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::common::{EmptyType, PlatformType, TrendingRes, TrendingsRes, not_empty_str, http_get};
use crate::errors::Result;

pub const TRENDING_ENDPOINT: &'static str =
  "https://newsapp.sina.cn/api/hotlist?newsId=HB-1-snhs/top_news_list-all";

pub async fn trending(client: &Client) -> Result<TrendingsRes> {
  http_get::<EmptyType, EmptyType, WeiboRes>(client, TRENDING_ENDPOINT, None, None, None)
    .await
    .map(|r| r.into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeiboRes {
  #[serde(rename = "status")]
  status: i32,

  #[serde(rename = "data")]
  data: WeiboData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeiboData {
  #[serde(rename = "hotList")]
  hot_list: Vec<WeiboHot>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeiboHot {
  #[serde(rename = "info")]
  info: WeiboInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeiboInfo {
  #[serde(rename = "title")]
  title: String,

  #[serde(rename = "hotValue")]
  hot_value: Option<String>,
}

impl From<WeiboInfo> for TrendingRes {
  fn from(value: WeiboInfo) -> Self {
    Self {
      url: format!(
        "https://m.weibo.cn/search?containerid=100103type%3D1%26q%3D%23{}%23",
        &value.title
      ),
      title: value.title,
      trend: not_empty_str(value.hot_value),
    }
  }
}

impl From<WeiboRes> for TrendingsRes {
  fn from(value: WeiboRes) -> Self {
    Self {
      platform: PlatformType::Weibo,
      trendings: value
        .data
        .hot_list
        .into_iter()
        .map(|r| r.info.into())
        .collect(),
    }
  }
}
