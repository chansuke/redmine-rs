use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, ArgMatches};

#[derive(Debug)]
pub struct Cli {}

impl Cli {
    pub fn from_args() -> ArgMatches {
        let matches = App::new("redmine")
            .version(crate_version!())
            .about(crate_description!())
            .author(crate_authors!())
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(
                App::new("get")
                    .about("show things")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(
                        App::new("issues") // Subcommands can have their own subcommands,
                            // which in turn have their own subcommands
                            .about("show issue")
                            .arg(
                                Arg::new("id")
                                    .required(true)
                                    .takes_value(true)
                                    .about("display the issue of given id"),
                            ),
                    )
                    .subcommand(
                        App::new("projects").about("show projects").arg(
                            Arg::new("id")
                                .required(true)
                                .takes_value(true)
                                .about("display the project of given id"),
                        ),
                    )
                    .subcommand(
                        App::new("users").about("show users").arg(
                            Arg::new("id")
                                .required(true)
                                .takes_value(true)
                                .about("display the user of given id"),
                        ),
                    )
                    .subcommand(
                        App::new("roles").about("show roles").arg(
                            Arg::new("id")
                                .required(true)
                                .takes_value(true)
                                .about("display the role of given id"),
                        ),
                    ),
            )
            .subcommand(
                App::new("list")
                    .about("list things")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(App::new("issues").about("show the list of issue"))
                    .subcommand(App::new("projects").about("show the list of project"))
                    .subcommand(App::new("users").about("show the list of user"))
                    .subcommand(
                        App::new("memberships").about("show the list of mebership of your project"),
                    )
                    .subcommand(App::new("news").about("show the list of news"))
                    .subcommand(App::new("trackers").about("show the list of trackers"))
                    .subcommand(App::new("roles").about("show the list of roles")),
            )
            .subcommand(
                App::new("post")
                    .about("post things")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(
                        App::new("issues").about("post the issue").arg(
                            Arg::new("issue_args")
                                .required(true)
                                .multiple(true)
                                .takes_value(true)
                                .about("Subject and description of an Issue"),
                        ),
                    ),
            )
            .get_matches();
        matches
    }
}
