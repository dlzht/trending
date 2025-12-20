//! ### Trending
//!
//! Trending is a library for retrieving trending information from media platforms. It currently supports the following platforms:
//!
//! | platform      | trending | search | site                        |
//! | :-----------: | :------: | :----: | :-------------------------- |
//! | hupu          | ✓        | -      | <https://m.hupu.com>        |
//! | tencent       | ✓        | -      | <https://news.qq.com>       |
//! | netease       | ✓        | ✓      | <https://m.163.com>         |
//! | tieba         | ✓        | -      | <https://www.tieba.com>     |
//! | toutiao       | ✓        | -      | <https://www.toutiao.com>   |
//! | weibo         | ✓        | -      | <https://weibo.com>         |
//! | zhihu         | ✓        | -      | <https://www.zhihu.com>     |
//!
//! ### Create AsyncClient
//!
//! ```rust
//! use std::time::Duration;
//! use trending::{client::AsyncClient, errors::Result};
//!
//! // new with default options
//! let client = AsyncClient::new();
//!
//! // or new with custom options
//! let options = ClientOptions::new().with_timeout(Duration::from_secs(5));
//! let client = AsyncClient::new_with_options(options);
//!
//! ```
//!
//! ### Trending Query
//!
//! ```rust
//! let res = client.trending_tencent().await?;
//! println!("receive {} trendings from {}", res.result.len(), res.platform);
//! for (index, trending) in res.result.iter().enumerate() {
//!   println!("{:2} -> {}", index, trending.title);
//! }
//! ```
//!
//! ### Search Query
//!
//! ```rust
//! let req = SearchReq::new("KEYWORD");
//! let res = client.search_netease(&req).await?;
//! println!("receive {} searches from {}", res.result.len(), res.platform);
//! for (index, search) in res.result.iter().enumerate() {
//!   println!("{:2} -> {}", index, search.title);
//! }
//!
//! ```

pub mod client;
pub mod common;
pub mod errors;
mod hupu;
mod netease;
mod tencent;
mod tieba;
mod toutiao;
mod weibo;
mod zhihu;
