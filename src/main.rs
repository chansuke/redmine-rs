mod backend;
mod cli;
mod config;
mod errors;
mod resources;
mod ui;
mod utils;

use anyhow::Result;

use crate::cli::Cli;
use crate::errors::RmError;
use crate::ui::printer::Printer;

#[tokio::main]
async fn main() -> Result<(), RmError> {
    let matches = Cli::from_args();
    let command = utils::command::extract_command(&matches);
    let sub_command = utils::command::extract_subcommand(&matches);
    let arg = utils::command::extract_arg(&matches);
    let endpoint = utils::endpoint::build_endpoint(sub_command, arg)?;
    let endpoint_with_apikey = utils::endpoint::append_apikey_clause(&endpoint)?;

    match sub_command {
        "issues" => match command {
            "get" => {
                let issue = backend::issues::get_issue(&endpoint).await?;
                Printer::print_result(issue);
            }
            "list" => {
                let issues = backend::issues::get_issues(&endpoint).await?;
                Printer::print_result(issues);
            }
            _ => unreachable!(),
        },
        "projects" => match command {
            "get" => {
                let project = backend::projects::get_project(&endpoint).await?;
                Printer::print_result(project);
            }
            "list" => {
                let projects = backend::projects::get_projects(&endpoint).await?;
                Printer::print_result(projects);
            }
            _ => unreachable!(),
        },
        "memberships" => match command {
            "list" => {
                let memberships = backend::memberships::get_memberships(&endpoint).await?;
                Printer::print_result(memberships);
            }
            _ => unreachable!(),
        },
        "users" => match command {
            "get" => {
                let user = backend::users::get_user(&endpoint).await?;
                Printer::print_result(user);
            }
            "list" => {
                let users = backend::users::get_users(&endpoint_with_apikey).await?;
                Printer::print_result(users);
            }
            _ => unreachable!(),
        },
        "news" => match command {
            "list" => {
                let news = backend::news::get_news(&endpoint).await?;
                Printer::print_result(news);
            }
            _ => unreachable!(),
        },
        "trackers" => match command {
            "list" => {
                let trackers = backend::trackers::get_trackers(&endpoint).await?;
                Printer::print_result(trackers);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
    Ok(())
}
