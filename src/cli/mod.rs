//! Handles parsing the CLI commands and options for invoking `doc-gen`.
//!
//! We will likely use `clap` for this.

use clap::{App, Arg, ArgMatches};

pub fn parse<'a>() -> ArgMatches<'a> {
    let app = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("check")
                .short("c")
                .long("check")
                .help("Validates existing documentation"),
        )
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .help("Sets the maximum number of threads to use")
                .takes_value(true)
                .validator(|s| {
                    s.parse::<usize>()
                        .map_err(|_| "Not an integer".into())
                        .and_then(|t| {
                            t.gt(&0)
                                .then(|| ())
                                .ok_or_else(|| "Number of threads must be greater than zero".into())
                        })
                }),
        );

    app.get_matches()
}
