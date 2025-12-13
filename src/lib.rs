pub mod common;
pub mod errors;
mod toutiao;
mod weibo;
mod zhihu;

use reqwest::{Client, Method, header::HeaderMap};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;

use crate::errors::{ReqwestClientSnafu, Result};

#[cfg(test)]
mod test {
  use crate::common::ClientOptions;

  #[tokio::test]
  async fn test() {
    let client = ClientOptions::new().build_client().unwrap();
    let res = client.trending_toutiao().await.unwrap();
    println!("{:#?}", res);
  }
}
