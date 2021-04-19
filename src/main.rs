use std::error::Error;
use std::result::Result;

mod ast;
mod check;
mod cli;
mod doc_config;
mod doc_gen;
mod doc_inserter;
mod doc_parser;
mod doc_tree;

fn main() {
    let args = cli::parse();

    // TODO(#19) check for the existence of the config file.
    // TODO(#3) deserialize config file.

    let max_threads = args
        .value_of("threads")
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(num_cpus::get);

    let run = if args.is_present("check") {
        try_validate
    } else {
        try_generate
    };

    if let Err(e) = run(max_threads) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// TODO(#3) pass DocConfig as an argument.
fn try_generate(_max_threads: usize) -> Result<(), Box<dyn Error>> {
    Err("Documentation generation is not implemented yet".into())
}

// TODO(#3) pass DocConfig as an argument.
fn try_validate(_max_threads: usize) -> Result<(), Box<dyn Error>> {
    Err("Documentation validation is not implemented yet".into())
}
