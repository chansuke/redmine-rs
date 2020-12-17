mod backend;
mod cli;
mod config;
mod resources;
mod utils;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Cli::from_args();

    let command = utils::command::extract_command(&matches);
    let sub_command = utils::command::extract_subcommand(&matches);
    let arg = utils::command::extract_arg(&matches);

    let endpoint = utils::endpoint::build_endpoint(sub_command, arg)?;
    let result = backend::issues::call_api(&endpoint, command).await?;

    println!("{:?}", result);
    Ok(())
}
