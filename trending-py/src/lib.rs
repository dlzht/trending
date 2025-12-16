use pyo3::prelude::*;

#[pymodule]
mod trending {

  use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    time::Duration,
  };

  use pyo3::{exceptions::PyOSError, prelude::*};
  use reqwest::{
    Proxy,
    header::{HeaderName, HeaderValue},
  };
  use snafu::ResultExt;
  use trending::{
    common::{
      BlockTrendingClient as RBlockClient, ClientOptions as RClientOptions,
      TrendingRes as RTrendingRes, TrendingsRes as RTrendingsRes,
    },
    errors::{
      ReqwestClientSnafu, ReqwestHeaderNameSnafu, ReqwestHeaderValueSnafu,
      TrendingError as RTrendingError,
    },
  };

  #[pyclass]
  #[derive(Debug)]
  pub struct TrendingError(RTrendingError);

  impl From<RTrendingError> for TrendingError {
    fn from(err: RTrendingError) -> Self {
      Self(err)
    }
  }

  impl From<TrendingError> for PyErr {
    fn from(err: TrendingError) -> Self {
      PyOSError::new_err(err.0.to_string())
    }
  }

  pub type Result<T> = std::result::Result<T, TrendingError>;

  #[pyclass(str)]
  #[derive(Debug, Clone)]
  pub struct ClientOptions {
    options: RClientOptions,
  }

  impl Display for ClientOptions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
    }
  }

  #[pymethods]
  impl ClientOptions {
    #[new]
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

    pub fn with_timeout(&mut self, timeout: Duration) {
      self.options.timeout = Some(timeout);
    }

    pub fn debug_print(&self) {
      println!("{:?}", self);
    }
  }

  #[pyclass]
  struct BlockTrendingClient {
    client: RBlockClient,
  }

  #[pyclass(str)]
  #[derive(Debug, Clone)]
  pub struct TrendingRes {
    #[pyo3(get, set)]
    title: String,

    #[pyo3(get, set)]
    url: String,

    #[pyo3(get, set)]
    trend: Option<String>,
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

  impl Display for TrendingRes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
    }
  }

  #[pyclass(str)]
  #[derive(Debug, Clone)]
  pub struct TrendingsRes {
    #[pyo3(get, set)]
    platform: String,

    #[pyo3(get, set)]
    trendings: Vec<TrendingRes>,
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

  impl Display for TrendingsRes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
    }
  }

  #[pymethods]
  impl BlockTrendingClient {
    #[new]
    #[pyo3(signature = (options = None))]
    fn new(options: Option<ClientOptions>) -> Result<Self> {
      let client = if let Some(options) = options {
        RBlockClient::new_with_options(options.options)?
      } else {
        RBlockClient::new()
      };
      Ok(Self { client })
    }

    pub fn trending_zhihu(&self) -> Result<TrendingsRes> {
      let res = self.client.trending_zhihu()?;
      Ok(res.into())
    }

    pub fn trending_weibo(&self) -> Result<TrendingsRes> {
      let res = self.client.trending_weibo()?;
      Ok(res.into())
    }

    pub fn trending_toutiao(&self) -> Result<TrendingsRes> {
      let res = self.client.trending_toutiao()?;
      Ok(res.into())
    }

    pub fn trending_tencent(&self) -> Result<TrendingsRes> {
      let res = self.client.trending_tencent()?;
      Ok(res.into())
    }

    pub fn trending_tieba(&self) -> Result<TrendingsRes> {
      let res = self.client.trending_tieba()?;
      Ok(res.into())
    }
  }
}
