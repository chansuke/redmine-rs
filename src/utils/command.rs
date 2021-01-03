use clap::ArgMatches;

pub(crate) fn extract_command(args: &ArgMatches) -> &str {
    let command = args.subcommand_name().unwrap();
    command
}

pub(crate) fn extract_subcommand(args: &ArgMatches) -> &str {
    let mut sub_command = "";

    match args.subcommand() {
        Some(("get", get_matches)) => {
            sub_command = get_matches.subcommand_name().unwrap();
        }
        Some(("list", get_matches)) => {
            sub_command = get_matches.subcommand_name().unwrap();
        }
        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => {} // If all subcommands are defined above, anything else is unreachabe!()
    }
    sub_command
}

pub(crate) fn extract_arg(args: &ArgMatches) -> &str {
    // Extract an argument of subcommand
    let mut arg = "";
    match args.subcommand() {
        Some(("get", get_matches)) => {
            if let Some(("issues", issues_matches)) = get_matches.subcommand() {
                arg = issues_matches.value_of("id").unwrap();
            }
            if let Some(("projects", projects_matches)) = get_matches.subcommand() {
                arg = projects_matches.value_of("id").unwrap();
            }
        }
        None => println!("No subcommand was used"),
        _ => {}
    }

    arg
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{App, Arg};

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn test_extract_command() {
        let test_matches = App::new("".to_string())
            .subcommand(App::new("".to_string()).subcommand(
                App::new("".to_string()).arg(Arg::new("").required(true).takes_value(true)),
            ))
            .get_matches();
        assert!(extract_command(&test_matches) == "get");
    }
}
