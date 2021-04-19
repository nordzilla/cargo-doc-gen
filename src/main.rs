use std::error::Error;
use std::result::Result;

use clap::ArgMatches;

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
    if let Err(e) = try_run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_run(_args: ArgMatches) -> Result<(), Box<dyn Error>> {
    Err("The program logic is not yet implemented.".into())
}
