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

    match sub_command {
        "issues" => match command {
            "get" => {
                backend::issues::get_issue(&endpoint).await?;
            }
            "list" => {
                backend::issues::get_issues(&endpoint).await?;
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    Ok(())
}
