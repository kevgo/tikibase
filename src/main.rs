//! the CLI wrapper around lib.rs

use clap::StructOpt;
use std::path::PathBuf;
use tikibase::{render_text, run, Args, Fix, Format, Issue};

fn main() {
    let args = Args::parse();
    let dir = PathBuf::from(".");
    let (issues, fixes) = run(args.command, dir);
    let exit_code = match args.format {
        Format::Text => {
            let (output, exit_code) = render_text(issues, fixes);
            for line in output {
                println!("{line}");
            }
            exit_code
        }
        Format::Json => print_json(issues, fixes),
    };
    std::process::exit(exit_code);
}

fn print_text(outcomes: Vec<String>) {
    for outcome in outcomes {
        println!("{outcome}");
    }
}

fn print_json(issues: Vec<Issue>, fixes: Vec<Fix>) -> i32 {
    0
}
