use tokio;
use trending::{common::TrendingClient, errors::Result};

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
  let client = TrendingClient::new();
  if let Ok(res) = client.trending_hupu().await {
    for trend in &res.trendings {
      println!("{:?}", trend);
    }
    println!("{}", res.trendings.len());
  }
  Ok(())
}
