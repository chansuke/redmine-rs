use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, ArgMatches};

#[derive(Debug)]
pub struct Cli {}

impl Cli {
    pub fn from_args() -> ArgMatches {
        let matches = App::new("redmine")
            .version(crate_version!())
            .about(crate_description!())
            .author(crate_authors!())
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
                                    .about("specify the number of your issue id"),
                            ),
                    )
                    .subcommand(
                        App::new("projects").about("show projects").arg(
                            Arg::new("id")
                                .required(true)
                                .takes_value(true)
                                .about("specify the number of your project id"),
                        ),
                    ),
            )
            .subcommand(
                App::new("list")
                    .about("list things")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .subcommand(App::new("issues").about("show the list of issue")),
            )
            .get_matches();
        matches
    }
}
