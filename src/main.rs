//! the CLI wrapper around lib.rs

use clap::StructOpt;
use tikibase::{cli, process};

fn main() {
    let args = cli::Args::parse();
    let (mut outcomes, exitcode) = process(&args.command, ".");
    outcomes.sort();
    match args.format {
        cli::Format::Text => print_text(outcomes),
        cli::Format::Json => print_json(outcomes),
    }
    std::process::exit(exitcode);
}

fn print_text(outcomes: Vec<String>) {
    for outcome in outcomes {
        println!("{outcome}");
    }
}

fn print_json(outcomes: Vec<String>) {}
