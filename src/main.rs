//! the CLI wrapper around lib.rs

use clap::StructOpt;
use std::io;
use std::path::PathBuf;
use tikibase::{render_text, run, Args, Format, Outcome};

fn main() {
    let args = Args::parse();
    let outcome = run(args.command, PathBuf::from("."));
    let exit_code = match args.format {
        Format::Text => print_text(outcome),
        Format::Json => print_json(outcome),
    };
    std::process::exit(exit_code);
}

fn print_text(outcome: Outcome) -> i32 {
    let (output, exit_code) = render_text(&outcome);
    for line in output {
        println!("{line}");
    }
    exit_code
}

fn print_json(outcome: Outcome) -> i32 {
    // NOTE: using a buffered writer doesn't seem to improve performance here
    if let Err(err) = serde_json::to_writer_pretty(io::stdout(), &outcome) {
        println!("Error serializing JSON: {}", err);
        return 1;
    }
    outcome.issues.len() as i32
}
