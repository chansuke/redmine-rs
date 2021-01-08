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
    let endpoint_with_apikey = utils::endpoint::append_apikey_clause(&endpoint)?;

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
        "projects" => match command {
            "get" => {
                backend::projects::get_project(&endpoint).await?;
            }
            "list" => {
                backend::projects::get_projects(&endpoint).await?;
            }
            _ => unreachable!(),
        },
        "users" => match command {
            "get" => {
                backend::users::get_user(&endpoint).await?;
            }
            "list" => {
                backend::users::get_users(&endpoint_with_apikey).await?;
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    Ok(())
}
