use clap::ArgMatches;

pub(crate) fn extract_command(args: &ArgMatches) -> &str {
    let command = args.subcommand_name().unwrap();
    command
}

pub(crate) fn extract_subcommand<'a, 'b>(args: &'a ArgMatches) -> &'a str {
    let mut sub_command = "";

    match args.subcommand() {
        Some(("get", get_matches)) => {
            sub_command = get_matches.subcommand_name().unwrap();
        }
        None => println!("No subcommand was used"), // If no subcommand was used it'll match the tuple ("", None)
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    sub_command
}

pub(crate) fn extract_arg<'a, 'b>(args: &'a ArgMatches) -> &'a str {
    // Extract an argument of subcommand
    let mut arg = "";

    match args.subcommand() {
        Some(("get", get_matches)) => match get_matches.subcommand() {
            Some(("issues", issues_matches)) => {
                arg = issues_matches.value_of("id").unwrap();
            }
            _ => unreachable!(),
        },
        None => println!("No subcommand was used"),
        _ => unreachable!(),
    }

    arg
}
