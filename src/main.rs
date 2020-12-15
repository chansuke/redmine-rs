mod cli;
mod config;
mod redmine;
mod utils;

use redmine::issue::Issue;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Cli::from_args();

    let _command = utils::command::extract_command(&matches);
    let sub_command = utils::command::extract_subcommand(&matches);
    let arg = utils::command::extract_arg(&matches);

    // Build endpoint.
    let endpoint = utils::endpoint::build_endpoint_url(sub_command, arg);
    let response = reqwest::get(&endpoint).await?.text().await?;
    let result: Issue = serde_json::from_str(&response)?;

    println!("{:?}", result);
    Ok(())
}
