//! the CLI wrapper around lib.rs

use clap::StructOpt;
use tikibase::{cli, process};

fn main() {
    let args = cli::Args::parse();
    let (mut outcomes, exitcode) = process(&args.command, ".");
    outcomes.sort();
    for outcome in outcomes {
        println!("{outcome}");
    }
    std::process::exit(exitcode);
}
