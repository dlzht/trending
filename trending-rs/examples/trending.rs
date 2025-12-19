use tokio;
use trending::{client::AsyncClient, common::SearchReq, errors::Result};

#[tokio::main(flavor = "current_thread")]
async fn main() {
  match run_main().await {
    Ok(_) => std::process::exit(0),
    Err(err) => {
      eprintln!("{}", err);
    }
  }
}

async fn run_main() -> Result<()> {
  let client = AsyncClient::new();
  let query: SearchReq = ("王力宏").into();
  if let Ok(res) = client.search_netease(&query).await {
    for item in &res.result {
      println!("{:?}", item);
    }
    println!("{}", res.result.len());
  }
  Ok(())
}
