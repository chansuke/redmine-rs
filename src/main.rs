pub mod redmine;

use crate::redmine::issue::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    base_url: String,
    api_key: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let base_url = args.base_url;
    let api_key = args.api_key;
    let cli = Cli {
        base_url: base_url.to_string(),
        api_key: api_key.to_string(),
    };
    let response = reqwest::get(&cli.base_url).await?.text().await?;
    let result: Issue = serde_json::from_str(&response)?;
    println!("{:?}", result.issue);
    Ok(())
}
