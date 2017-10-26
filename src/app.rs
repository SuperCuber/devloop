use clap::{ArgMatches, App, AppSettings, Arg};

pub fn parse_args() -> ArgMatches<'static> {
    App::new("Devloop")
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("file")
                .help("Configuration file")
                .default_value("dev_loop"),
        )
        .get_matches()
}
